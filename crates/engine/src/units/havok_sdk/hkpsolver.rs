//! `Havok SDK/hkpsolver.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vector code is SSE: every lane is a single-precision operation, bit for bit as the exe
//! does it. The scalar parts that go through the x87 stack (`FLD` / `FMUL` / `FSTP`) load
//! through `x87_quiet`, compute in `double` and round to `float` where the exe stores.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, load_bits, load_vec, shufps, store_vec, sum3, zip_with, Vec4,
};

fn splat(value: f32) -> Vec4 {
    [value; 4]
}

fn add(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x + y)
}

fn sub(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x - y)
}

fn mul(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x * y)
}

/// The table `hkSolver` indexes by a signed byte (`011b9434`).
const BYTE_TABLE: u32 = 0x011b_9434;

// Translated from 00d892d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolveUpload` (engine map): a bare `RET`.
pub fn hk_solve_upload(_e: &mut Engine) {}

// Translated from 00d892f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpsolver.cpp`): integrates one solver body's velocity vectors. Register inputs:
/// `body` in EAX, `damping` in EDX and `gravity` in ESI; `input` and `damp` are the two stack
/// words (caller cleans up; `damp` is read as a byte). With `step = body[+8]`, `a = body[+0x10]
/// - body[+0x40]` and `b = body[+0x20] - body[+0x50]`, `k = input[+0x30]`:
/// `body[+0x40] += k * a` and `body[+0x50] += k * b`. When `damp` is zero then
/// `body[+0x20] = body[+0x50] + b` and `body[+0x10] = (a + body[+0x40]) + step * gravity`;
/// otherwise `body[+0x40]` and `body[+0x50]` are multiplied lane by lane by `damping` and
/// `body[+0x20] = b`, `body[+0x10] = a`.
pub fn fn_00d892f0(e: &mut Engine, body: Ptr, damping: Ptr, gravity: Ptr, input: Ptr, damp: u8) {
    let body = body.addr();
    let step = splat(e.mem.f32(body + 8));
    let scaled_gravity = mul(step, load_vec(e, gravity.addr()));
    let a = sub(load_vec(e, body + 0x10), load_vec(e, body + 0x40));
    let b = sub(load_vec(e, body + 0x20), load_vec(e, body + 0x50));
    let factor = splat(e.mem.f32(input.addr() + 0x30));
    let position = add(mul(factor, a), load_vec(e, body + 0x40));
    store_vec(e, body + 0x40, position);
    let factor = splat(e.mem.f32(input.addr() + 0x30));
    let rotation = add(mul(factor, b), load_vec(e, body + 0x50));
    store_vec(e, body + 0x50, rotation);
    if damp == 0 {
        let angular = add(load_vec(e, body + 0x50), b);
        let linear = add(add(a, load_vec(e, body + 0x40)), scaled_gravity);
        store_vec(e, body + 0x20, angular);
        store_vec(e, body + 0x10, linear);
    } else {
        let damping = damping.addr();
        let scaled = mul(load_vec(e, body + 0x40), load_vec(e, damping));
        store_vec(e, body + 0x40, scaled);
        let scaled = mul(load_vec(e, damping), load_vec(e, body + 0x50));
        store_vec(e, body + 0x50, scaled);
        store_vec(e, body + 0x20, b);
        store_vec(e, body + 0x10, a);
    }
}

// Translated from 00d893a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolver_solveStiffSpringChain` (engine map), cdecl `(solver_info, _unused_1, bodies, chain,
/// impulses)`. `chain` holds the link count `n` (16-bit at `+2`), the floats `a` (`+8`) and `b`
/// (`+0xc`), then `n` Jacobian records of 0x30 bytes (three vectors) from `+0x10`, `n` records
/// of three floats (the elimination coefficients), `n + 1` floats (the solution `T`, with
/// `T[-1] = 0`) and `n + 1` body offsets (added to `bodies`). The forward pass computes
/// `T[i] = (a * J.w - sum(R) - T[i - 1] * S0) / S1`, where `R` combines the bodies' velocity
/// and position vectors with the Jacobian rows and `s = solver_info[+0x40] * a`; the backward
/// pass turns it into impulses `lambda = T[i] - S2 * lambda_previous`, applies them to the
/// two bodies (their vectors at `+0x10`, `+0x20`) and adds each `lambda` to `impulses[i]`.
pub fn hk_solver_solve_stiff_spring_chain(
    e: &mut Engine,
    solver_info: u32,
    _unused_1: u32,
    bodies: u32,
    chain: u32,
    impulses: u32,
) {
    let count = u32::from(e.mem.u16(chain + 2));
    let jacobians = chain + 0x10;
    let coefficients = chain + 0x10 + 0x30 * count;
    let solutions = chain + 0x14 + 0x3c * count;
    let pairs = chain + 0x14 + 0x40 * count;
    // `FST` of the `FLDZ` the function starts with: T[-1] = 0.
    e.mem.set_f32(solutions - 4, 0.0);
    for index in 0..count {
        let body0 = e.mem.u32(pairs + 4 * index).wrapping_add(bodies);
        let body1 = e.mem.u32(pairs + 4 * index + 4).wrapping_add(bodies);
        let a = x87_quiet(e.mem.f32(chain + 8));
        let b = e.mem.f32(chain + 0xc);
        let row_a = load_vec(e, jacobians + 0x30 * index);
        let row_b = load_vec(e, jacobians + 0x30 * index + 0x10);
        let row_c = load_vec(e, jacobians + 0x30 * index + 0x20);
        let angular1 = sub(load_vec(e, body1 + 0x20), load_vec(e, body1 + 0x50));
        let angular0 = sub(load_vec(e, body0 + 0x20), load_vec(e, body0 + 0x50));
        let weighted = add(mul(angular1, row_c), mul(angular0, row_b));
        let linear1 = sub(load_vec(e, body1 + 0x10), load_vec(e, body1 + 0x40));
        let linear0 = sub(load_vec(e, body0 + 0x10), load_vec(e, body0 + 0x40));
        let relative = sub(linear0, linear1);
        let first = mul(add(weighted, mul(relative, row_a)), splat(b));
        let factor = x87_quiet(e.mem.f32(solver_info + 0x40));
        let scale = (f64::from(factor) * f64::from(a)) as f32;
        let second = add(
            add(
                mul(load_vec(e, body1 + 0x50), row_c),
                mul(load_vec(e, body0 + 0x50), row_b),
            ),
            mul(
                sub(load_vec(e, body0 + 0x40), load_vec(e, body1 + 0x40)),
                row_a,
            ),
        );
        let sum = sum3(add(first, mul(second, splat(scale))));
        let right = f64::from(a) * f64::from(x87_quiet(row_a[3])) - f64::from(sum);
        let right = right as f32;
        let record = coefficients + 0xc * index;
        let previous = x87_quiet(e.mem.f32(solutions + 4 * index - 4));
        let coefficient_0 = x87_quiet(e.mem.f32(record));
        let coefficient_1 = x87_quiet(e.mem.f32(record + 4));
        let solution = (f64::from(x87_quiet(right))
            - f64::from(previous) * f64::from(coefficient_0))
            / f64::from(coefficient_1);
        e.mem.set_f32(solutions + 4 * index, solution as f32);
    }
    let mut previous_lambda = 0.0f32;
    for index in (0..count).rev() {
        let body0 = e.mem.u32(pairs + 4 * index).wrapping_add(bodies);
        let body1 = e.mem.u32(pairs + 4 * index + 4).wrapping_add(bodies);
        let solution = x87_quiet(e.mem.f32(solutions + 4 * index));
        let coefficient_2 = x87_quiet(e.mem.f32(coefficients + 0xc * index + 8));
        let lambda =
            (f64::from(solution) - f64::from(coefficient_2) * f64::from(previous_lambda)) as f32;
        let impulse0 = mul(load_vec(e, body0 + 0x30), splat(lambda));
        let impulse1 = mul(load_vec(e, body1 + 0x30), splat(lambda));
        let velocity0 = load_vec(e, body0 + 0x10);
        let row_0 = load_vec(e, jacobians + 0x30 * index);
        store_vec(
            e,
            body0 + 0x10,
            add(mul(splat(impulse0[3]), row_0), velocity0),
        );
        let velocity1 = sub(load_vec(e, body1 + 0x10), mul(splat(impulse1[3]), row_0));
        store_vec(e, body1 + 0x10, velocity1);
        let row_1 = load_vec(e, jacobians + 0x30 * index + 0x10);
        let angular0 = add(mul(row_1, impulse0), load_vec(e, body0 + 0x20));
        let row_2 = load_vec(e, jacobians + 0x30 * index + 0x20);
        let angular1_old = load_vec(e, body1 + 0x20);
        store_vec(e, body0 + 0x20, angular0);
        store_vec(e, body1 + 0x20, add(mul(row_2, impulse1), angular1_old));
        let slot = impulses + 4 * index;
        let total = e.mem.f32(slot) + lambda;
        e.mem.set_f32(slot, total);
        previous_lambda = lambda;
    }
}

// Translated from 00d89ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpsolver.cpp`), cdecl `(solver_info, begin, end)`: walks the 0x80-byte solver
/// elements from `begin` while the address is below `end` (an `end` of 0 means no limit). Each
/// element's first byte is its type: type 0 and types 1 and 2 clear the 32 bytes at `+0x40`;
/// type 0 also adds `element[+8] * solver_info[+0x10 ..]` (a 4-lane vector) to the vector at
/// `+0x10`; type 3 ends the walk. Any other type reaches the exe's `INT3` (at `00d89f64`).
pub fn fn_00d89ed0(e: &mut Engine, solver_info: u32, begin: u32, end: u32) {
    let velocity = load_vec(e, solver_info + 0x10);
    let end = if end == 0 { u32::MAX } else { end };
    let mut element = begin;
    while element < end {
        match e.mem.u8(element) {
            0 => loop {
                let step = splat(e.mem.f32(element + 8));
                clear_solver_element(e, element);
                let moved = add(mul(step, velocity), load_vec(e, element + 0x10));
                store_vec(e, element + 0x10, moved);
                element = element.wrapping_add(0x80);
                if e.mem.u8(element) != 0 {
                    break;
                }
                if element >= end {
                    return;
                }
            },
            1 | 2 => {
                clear_solver_element(e, element);
                element = element.wrapping_add(0x80);
            }
            3 => return,
            other => panic!("INT3 at 00d89f64: solver element type {other}"),
        }
    }
}

fn clear_solver_element(e: &mut Engine, element: u32) {
    store_vec(e, element + 0x40, [0.0; 4]);
    store_vec(e, element + 0x50, [0.0; 4]);
}

/// The four-lane `0x7fffffff` mask (`01017d10`), the four-lane `1e6` (`010d3da0`), the zero
/// vector (`01268370`) and the vector `fn_00d89f80` adds a scaled step of to the velocities of
/// the types 1 and 2 (`01267e30`).
const ABS_MASK: u32 = 0x0101_7d10;
const LARGE_VELOCITY: u32 = 0x010d_3da0;
const ZERO_VECTOR: u32 = 0x0126_8370;
const CONSTANT_ACCELERATION: u32 = 0x0126_7e30;

/// The velocity clamp of `fn_00d89f80` for the element's two velocity vectors (`+0x10` linear,
/// `+0x20` angular), with the limits record at `params + 0x64 + 28 * element[+4]` (floats
/// `m0`, `m1`, the damping factor `f`, the stop threshold `t`): lanes 0..2 of both vectors must
/// be numbers below `1e6` in magnitude, else both are replaced by the zero vector; then, with
/// `s = m1 * |angular| + m0 * |linear|` (computed from the values before any replacement), when
/// `s <= params[0]` in all three lanes they are zeroed if `s <= t` in all three lanes and
/// scaled by `f` otherwise.
fn limit_velocities(e: &mut Engine, params: u32, element: u32) {
    let record = params
        .wrapping_add(0x64)
        .wrapping_add(e.mem.u32(element + 4).wrapping_mul(28));
    let linear = load_vec(e, element + 0x10);
    let angular = load_vec(e, element + 0x20);
    let mask = load_bits(e, ABS_MASK);
    let absolute = |vector: Vec4| -> Vec4 {
        std::array::from_fn(|lane| f32::from_bits(vector[lane].to_bits() & mask[lane]))
    };
    let (abs_linear, abs_angular) = (absolute(linear), absolute(angular));
    let first = e.mem.f32(record);
    let second = e.mem.f32(record + 4);
    let large = load_vec(e, LARGE_VELOCITY);
    let sum = add(
        mul(splat(second), abs_angular),
        mul(splat(first), abs_linear),
    );
    let sane = (0..3).all(|lane| {
        angular[lane] == angular[lane]
            && abs_angular[lane] < large[lane]
            && linear[lane] == linear[lane]
            && abs_linear[lane] < large[lane]
    });
    if !sane {
        copy_vec(e, ZERO_VECTOR, element + 0x10);
        copy_vec(e, ZERO_VECTOR, element + 0x20);
    }
    let low_limit = e.mem.f32(params);
    if !(0..3).all(|lane| sum[lane] <= low_limit) {
        return;
    }
    let stop_threshold = e.mem.f32(record + 0xc);
    if (0..3).all(|lane| sum[lane] <= stop_threshold) {
        store_vec(e, element + 0x20, [0.0; 4]);
        store_vec(e, element + 0x10, [0.0; 4]);
    } else {
        let factor = splat(e.mem.f32(record + 8));
        let angular = mul(factor, load_vec(e, element + 0x20));
        store_vec(e, element + 0x20, angular);
        let factor = splat(e.mem.f32(record + 8));
        let linear = mul(factor, load_vec(e, element + 0x10));
        store_vec(e, element + 0x10, linear);
    }
}

/// The integration step of `fn_00d89f80` for one element. With `a = linear - element[+0x40]` and
/// `b = angular - element[+0x50]`, `k = params[+0x30]`: `element[+0x40] += k * a` and
/// `element[+0x50] += k * b`; unless `last`, the linear velocity becomes `(a + element[+0x40]) +
/// element[+8] * acceleration` and the angular `b + element[+0x50]`; when `last`, both position
/// vectors are multiplied by `damping` and the velocities become `a` and `b`.
fn integrate_element(
    e: &mut Engine,
    params: u32,
    element: u32,
    acceleration: Vec4,
    last: bool,
    damping: f32,
) {
    let step = splat(e.mem.f32(element + 8));
    let scaled_acceleration = mul(step, acceleration);
    let a = sub(load_vec(e, element + 0x10), load_vec(e, element + 0x40));
    let b = sub(load_vec(e, element + 0x20), load_vec(e, element + 0x50));
    let factor = splat(e.mem.f32(params + 0x30));
    let position = add(mul(factor, a), load_vec(e, element + 0x40));
    store_vec(e, element + 0x40, position);
    let factor = splat(e.mem.f32(params + 0x30));
    let rotation = add(mul(factor, b), load_vec(e, element + 0x50));
    store_vec(e, element + 0x50, rotation);
    let (linear, angular) = if last {
        let damp = splat(damping);
        let position = mul(damp, load_vec(e, element + 0x40));
        store_vec(e, element + 0x40, position);
        let rotation = mul(damp, load_vec(e, element + 0x50));
        store_vec(e, element + 0x50, rotation);
        (a, b)
    } else {
        let angular = add(b, load_vec(e, element + 0x50));
        let linear = add(add(a, load_vec(e, element + 0x40)), scaled_acceleration);
        (linear, angular)
    };
    store_vec(e, element + 0x10, linear);
    store_vec(e, element + 0x20, angular);
}

// Translated from 00d89f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpsolver.cpp`), cdecl `(params, index, begin, end, reached_last)`: integrates the
/// 0x80-byte solver elements from `begin` while the address is below `end` (0: no limit) and
/// always answers `1`. The byte at `reached_last` is set to 1 when `index >= params[+0x114] - 1`
/// (signed), else 0; the elements are then damped instead of advanced. `damping` is
/// `params[+0x120] * params[+0x40]` (through the x87 stack). Element types (first byte): 0 and 2
/// run `limit_velocities` first; 0 uses `params[+0x10 ..]` as the acceleration and keeps going
/// over a run of type-0 elements, 1 and 2 use the vector at `01267e30`; 3 ends the walk. Any
/// other type leaves the exe spinning on the same element for ever (`00d8a008` jumps to the
/// loop test without advancing), so it panics here.
pub fn fn_00d89f80(
    e: &mut Engine,
    params: u32,
    index: i32,
    begin: u32,
    end: u32,
    reached_last: u32,
) -> u32 {
    let own_acceleration = load_vec(e, params + 0x10);
    let damping = (f64::from(x87_quiet(e.mem.f32(params + 0x120)))
        * f64::from(x87_quiet(e.mem.f32(params + 0x40)))) as f32;
    e.mem.set_u8(reached_last, 0);
    let last = index >= e.mem.i32(params + 0x114).wrapping_sub(1);
    if last {
        e.mem.set_u8(reached_last, 1);
    }
    let limit = if end == 0 { u32::MAX } else { end };
    let mut element = begin;
    while element < limit {
        match e.mem.u8(element) {
            0 => loop {
                limit_velocities(e, params, element);
                integrate_element(e, params, element, own_acceleration, last, damping);
                element = element.wrapping_add(0x80);
                if e.mem.u8(element) != 0 {
                    break;
                }
                if element >= limit {
                    return 1;
                }
            },
            1 => {
                let acceleration = load_vec(e, CONSTANT_ACCELERATION);
                integrate_element(e, params, element, acceleration, last, damping);
                element = element.wrapping_add(0x80);
            }
            2 => {
                limit_velocities(e, params, element);
                let acceleration = load_vec(e, CONSTANT_ACCELERATION);
                integrate_element(e, params, element, acceleration, last, damping);
                element = element.wrapping_add(0x80);
            }
            3 => return 1,
            other => panic!("00d8a008: solver element type {other}: the exe loops here for ever"),
        }
    }
    1
}
// Translated from 00d921c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpsolver.cpp`), cdecl `(destination, source)`: copies one byte.
pub fn fn_00d921c0(e: &mut Engine, destination: u32, source: u32) {
    let value = e.mem.u8(source);
    e.mem.set_u8(destination, value);
}

// Translated from 00d921d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpsolver.cpp`), cdecl `(source)`: the byte of the table at `011b9434` indexed by
/// the signed byte at `source`.
pub fn fn_00d921d0(e: &mut Engine, source: u32) -> u8 {
    let index = i32::from(e.mem.i8(source));
    e.mem.u8(BYTE_TABLE.wrapping_add(index as u32))
}

// Translated from 00d89690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolver_solveBallAndSocketChain` (engine map), cdecl `(solver_info, _unused_1, bodies,
/// chain, results)`. `chain` holds the link count `n` (16-bit at `+2`), the floats `c8` (`+8`) and
/// `c12` (`+0xc`), then `n` link records of nine vectors `v0 .. v8` from `+0x10`, `n` factor
/// records of nine vectors `w0 .. w8`, `n + 1` right-hand-side vectors `t` (with `t[0]` set to
/// zero) and `n + 1` body offsets (added to `bodies`).
///
/// The forward pass solves, for link `i` between bodies `a` and `b` (offsets `i` and `i + 1`):
/// with `F = solver_info[+0x40] * c8` (x87) the three residuals
/// `r_k = c8 * v[3k].w - sum3(P_k * F + Q_k * c12)` where
/// `P_k = v[3k + 1] * a[+0x50] + v[3k + 2] * b[+0x50] + (a[+0x40] - b[+0x40]) * v[3k]` and
/// `Q_k = v[3k + 1] * (a[+0x20] - a[+0x50]) + v[3k + 2] * (b[+0x20] - b[+0x50]) + v[3k] * e`
/// (`e` the difference of the bodies' `+0x10 - +0x40` vectors), form `r = (r_0, r_1, r_2, 0)` and
/// store `t[i + 1] = (r - (t[i].x * w0 + t[i].y * w1 + t[i].z * w2))` combined with
/// `w3 .. w5` the same way. The backward pass starts from a zero vector `y` and, for `i` from
/// `n - 1` down, takes `y = t[i + 1] - (y.x * w6 + y.y * w7 + y.z * w8)`; for each component `c`
/// of `y` it scales the bodies' `+0x30` vectors by it, adds `m_a.w * v[3c]` to `a[+0x10]` and
/// subtracts `m_b.w * v[3c]` from `b[+0x10]`, adds `v[3c + 1] * m_a` to `a[+0x20]` and
/// `v[3c + 2] * m_b` to `b[+0x20]`, and adds the component to `results[3i + c]`.
pub fn hk_solver_solve_ball_and_socket_chain(
    e: &mut Engine,
    solver_info: u32,
    _unused_1: u32,
    bodies: u32,
    chain: u32,
    results: u32,
) {
    let count = u32::from(e.mem.u16(chain + 2));
    let links = chain + 0x10;
    let factors = chain + 0x10 + 0x90 * count;
    let rights = chain + 0x10 + 0x120 * count;
    let pairs = chain + 0x20 + 0x130 * count;
    store_vec(e, rights, [0.0; 4]);
    for index in 0..count {
        let body_a = e.mem.u32(pairs + 4 * index).wrapping_add(bodies);
        let body_b = e.mem.u32(pairs + 4 * index + 4).wrapping_add(bodies);
        let link = links + 0x90 * index;
        let factor = factors + 0x90 * index;
        // `F = solver_info[+0x40] * c8` through the x87 stack; `c8` is loaded again for every
        // residual.
        let step = x87_quiet(e.mem.f32(chain + 8));
        let scale = (f64::from(x87_quiet(e.mem.f32(solver_info + 0x40))) * f64::from(step)) as f32;
        let bias = e.mem.f32(chain + 0xc);
        let v: [Vec4; 9] = std::array::from_fn(|k| load_vec(e, link + 0x10 * k as u32));
        let w: [Vec4; 6] = std::array::from_fn(|k| load_vec(e, factor + 0x10 * k as u32));
        let [linear_a, angular_a, position_a, rotation_a] =
            [0x10, 0x20, 0x40, 0x50].map(|offset| load_vec(e, body_a + offset));
        let [linear_b, angular_b, position_b, rotation_b] =
            [0x10, 0x20, 0x40, 0x50].map(|offset| load_vec(e, body_b + offset));
        let angular_delta_a = sub(angular_a, rotation_a);
        let angular_delta_b = sub(angular_b, rotation_b);
        let relative = sub(sub(linear_a, position_a), sub(linear_b, position_b));
        let offset = sub(position_a, position_b);
        let residual = |p: Vec4, q: Vec4, coefficient: f32| -> f32 {
            let sum = sum3(add(mul(p, splat(scale)), mul(q, splat(bias))));
            let product = f64::from(step) * f64::from(x87_quiet(coefficient));
            (product - f64::from(sum)) as f32
        };
        let first = residual(
            add(
                add(mul(v[1], rotation_a), mul(v[2], rotation_b)),
                mul(offset, v[0]),
            ),
            add(
                add(mul(v[1], angular_delta_a), mul(v[2], angular_delta_b)),
                mul(v[0], relative),
            ),
            v[0][3],
        );
        let second = residual(
            add(
                add(mul(v[5], rotation_b), mul(v[4], rotation_a)),
                mul(offset, v[3]),
            ),
            add(
                add(mul(angular_delta_a, v[4]), mul(v[5], angular_delta_b)),
                mul(v[3], relative),
            ),
            v[3][3],
        );
        let third = residual(
            add(
                add(mul(v[7], rotation_a), mul(v[8], rotation_b)),
                mul(offset, v[6]),
            ),
            add(
                add(mul(v[7], angular_delta_a), mul(v[8], angular_delta_b)),
                mul(v[6], relative),
            ),
            v[6][3],
        );
        let previous = load_vec(e, rights + 0x10 * index);
        let carried = add(
            add(mul(splat(previous[1]), w[1]), mul(splat(previous[0]), w[0])),
            mul(splat(previous[2]), w[2]),
        );
        let rest = sub([first, second, third, 0.0], carried);
        let next = add(
            add(mul(splat(rest[1]), w[4]), mul(splat(rest[0]), w[3])),
            mul(splat(rest[2]), w[5]),
        );
        store_vec(e, rights + 0x10 * (index + 1), next);
    }
    let mut previous = [0.0f32; 4];
    for index in (0..count).rev() {
        let body_a = e.mem.u32(pairs + 4 * index).wrapping_add(bodies);
        let body_b = e.mem.u32(pairs + 4 * index + 4).wrapping_add(bodies);
        let link = links + 0x90 * index;
        let factor = factors + 0x90 * index;
        let correction = add(
            add(
                mul(load_vec(e, factor + 0x60), splat(previous[0])),
                mul(load_vec(e, factor + 0x70), splat(previous[1])),
            ),
            mul(load_vec(e, factor + 0x80), splat(previous[2])),
        );
        let solution = sub(load_vec(e, rights + 0x10 * (index + 1)), correction);
        for component in 0..3u32 {
            let rows: [Vec4; 3] =
                std::array::from_fn(|k| load_vec(e, link + 0x10 * (3 * component + k as u32)));
            // The second component stores the first body's `+0x20` vector before the second
            // body's, the other two in the opposite order.
            apply_chain_component(
                e,
                body_a,
                body_b,
                rows,
                solution[component as usize],
                component == 1,
            );
            let slot = results + 12 * index + 4 * component;
            let total = e.mem.f32(slot) + solution[component as usize];
            e.mem.set_f32(slot, total);
        }
        previous = solution;
    }
}

/// One component's update of the two bodies in `hkSolver_solveBallAndSocketChain`: with `m_a` and
/// `m_b` the bodies' `+0x30` vectors times `value`, `a[+0x10] += m_a.w * rows[0]`,
/// `b[+0x10] -= m_b.w * rows[0]`, `a[+0x20] += rows[1] * m_a` and `b[+0x20] += rows[2] * m_b`;
/// `angular_a_first` is the order of the last two stores.
fn apply_chain_component(
    e: &mut Engine,
    body_a: u32,
    body_b: u32,
    rows: [Vec4; 3],
    value: f32,
    angular_a_first: bool,
) {
    let moment_a = mul(load_vec(e, body_a + 0x30), splat(value));
    let moment_b = mul(load_vec(e, body_b + 0x30), splat(value));
    let linear_a = add(mul(splat(moment_a[3]), rows[0]), load_vec(e, body_a + 0x10));
    store_vec(e, body_a + 0x10, linear_a);
    let linear_b = sub(load_vec(e, body_b + 0x10), mul(splat(moment_b[3]), rows[0]));
    store_vec(e, body_b + 0x10, linear_b);
    let angular_a = add(mul(rows[1], moment_a), load_vec(e, body_a + 0x20));
    let angular_b = add(mul(rows[2], moment_b), load_vec(e, body_b + 0x20));
    if angular_a_first {
        store_vec(e, body_a + 0x20, angular_a);
        store_vec(e, body_b + 0x20, angular_b);
    } else {
        store_vec(e, body_b + 0x20, angular_b);
        store_vec(e, body_a + 0x20, angular_a);
    }
}

/// The unnamed routines `hkSolver_solvePoweredChain` calls, all cdecl and called by address.
/// `00df0d30(context, rows)`.
const FN_00DF0D30: u32 = 0x00df_0d30;
/// `00df0810(first, second, flags, rows)`.
const FN_00DF0810: u32 = 0x00df_0810;
/// `00df11a0(context, vectors)`.
const FN_00DF11A0: u32 = 0x00df_11a0;
/// `00df1430(context, &found, &second, &mode_flag, flags)`.
const FN_00DF1430: u32 = 0x00df_1430;
/// `00df1260(context, &found, &second, &mode_flag, flags)`.
const FN_00DF1260: u32 = 0x00df_1260;
/// `00df2040(found, count, flags, matrices)`.
const FN_00DF2040: u32 = 0x00df_2040;
/// `00df1060(found, second, context, rows)`.
const FN_00DF1060: u32 = 0x00df_1060;

// Translated from 00d8a3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolver_solvePoweredChain` (engine map), cdecl `(solver_info, step, bodies, chain,
/// results)`; `step` points at a block with a `float` at `+4`.
///
/// Layout of `chain` (`n` is the 16-bit link count at `+4`): the floats at `+0x18` and `+0x1c`;
/// `n` link records of 0x90 bytes (nine vectors) from `+0x20`; `n` second records of 0x60
/// bytes (six vectors); `n` matrix blocks of 0x3c0 bytes (twelve vectors are used, from
/// `+0x180`); a 0x20-byte header and `n` vector pairs; `n` rows of eight floats (written from
/// `+0x10`); a table of `n + 1` body offsets; and `n` flag records of 0x40 bytes (the first
/// byte holds three two-bit modes, the floats at `+0xc`, `+0x20` and `+0x34` are sources).
///
/// The function builds a twelve-word context block on the stack (`solver_info`, `n`, the two
/// floats at `+0x18` and `+0x1c` copied through the x87 stack, and the region addresses and
/// `bodies`) and calls the unnamed routines in the constants above with it. First each flag
/// record adds `source * step[+4]` (x87) to the `w` lanes of three vectors of its second record.
/// Then `00df0d30` runs and for each link and each of its three modes a mode of 2 stores `0.0`
/// in the row block and any other nonzero mode calls `00df0810`. Two search loops follow
/// (`00df1430` with `00df2040` and `00df1060`, then `00df1260` with `00df2040` and `00df0810`);
/// every search is preceded by the refresh `00df11a0` and starts with its three outputs reset
/// to `-1`, `-1` and `0`, and the loop runs while the first output is not negative. Last the
/// backward pass walks the vector pairs `(Y0, Y1)` from the last link down: with the previous
/// solutions `(P, Q)` (zero at first) and the link's matrix rows `m0 .. m11`,
/// `Y0' = Y0 - (((m4 Q.y + m3 Q.x) + m5 Q.z) + ((m1 P.y + P.x m0) + m2 P.z))` and
/// `Y1' = Y1 - ((((m10 Q.y + m9 Q.x) + m11 Q.z)) + ((m7 P.y + P.x m6) + m8 P.z))`. The
/// components of `Y0'` update the two bodies like the ball-and-socket chain does (rows from the
/// link record), the components of `Y1'` add `(Y1'.c * body[+0x30]) * row` to the bodies' `+0x20`
/// vectors (rows from the second record), and every component is added to `results`.
pub fn hk_solver_solve_powered_chain(
    e: &mut Engine,
    solver_info: u32,
    step: u32,
    bodies: u32,
    chain: u32,
    results: u32,
) {
    let count = u32::from(e.mem.u16(chain + 4));
    let links = chain + 0x20;
    let seconds = chain + 0x20 + 0x90 * count;
    let pairs = chain + 0x40 + 0x4f0 * count;
    let matrices = chain + 0x20 + 0xf0 * count;
    let flags = chain + 0x44 + 0x4f4 * count;
    let vectors = chain + 0x20 + 0x4b0 * count;
    let rows = chain + 0x40 + 0x4d0 * count;
    e.with_stack(0x80, |e, frame| {
        // The stack frame: the three search outputs at `+0`, `+4` and `+0x20`, and the
        // context block at `+0x44`.
        let base = frame.addr();
        let found = base;
        let second = base + 4;
        let mode_flag = base + 0x20;
        let context = base + 0x44;
        e.mem.set_u32(context, solver_info);
        e.mem.set_u32(context + 4, count);
        let first_float = x87_quiet_bits(e.mem.u32(chain + 0x18));
        e.mem.set_u32(context + 8, first_float);
        let second_float = x87_quiet_bits(e.mem.u32(chain + 0x1c));
        e.mem.set_u32(context + 0xc, second_float);
        e.mem.set_u32(context + 0x10, links);
        e.mem.set_u32(context + 0x14, seconds);
        e.mem.set_u32(context + 0x18, pairs);
        e.mem.set_u32(context + 0x1c, bodies);
        e.mem.set_u32(context + 0x20, matrices);
        e.mem.set_u32(context + 0x24, flags);
        e.mem.set_u32(context + 0x28, vectors);
        e.mem.set_u32(context + 0x2c, rows);
        for index in 0..count {
            for (source, target) in [(0xc, 0x1c), (0x20, 0x3c), (0x34, 0x5c)] {
                let value = x87_quiet(e.mem.f32(flags + 0x40 * index + source));
                let factor = x87_quiet(e.mem.f32(step + 4));
                let slot = seconds + 0x60 * index + target;
                let sum =
                    f64::from(value) * f64::from(factor) + f64::from(x87_quiet(e.mem.f32(slot)));
                e.mem.set_f32(slot, sum as f32);
            }
        }
        e.call(FN_00DF0D30, &args![context, rows]);
        for index in 0..count {
            for component in 0..3u32 {
                let mode = (e.mem.u8(flags + 0x40 * index) >> (component * 2)) & 3;
                if mode == 2 {
                    e.mem
                        .set_f32(rows + 0x10 + 4 * (component + 8 * index), 0.0);
                } else if mode != 0 {
                    e.call(FN_00DF0810, &args![index, component, flags, rows]);
                }
            }
        }
        // Each search starts from a refreshed pair list with all outputs reset.
        let search = |e: &mut Engine, routine: u32| -> i32 {
            e.call(FN_00DF11A0, &args![context, vectors]);
            e.mem.set_u32(mode_flag, 0);
            e.mem.set_u32(found, u32::MAX);
            e.mem.set_u32(second, u32::MAX);
            e.call(routine, &args![context, found, second, mode_flag, flags]);
            e.mem.i32(found)
        };
        let mut index = search(e, FN_00DF1430);
        while index >= 0 {
            e.call(FN_00DF2040, &args![index, count, flags, matrices]);
            let other = e.mem.u32(second);
            e.call(FN_00DF1060, &args![index, other, context, rows]);
            index = search(e, FN_00DF1430);
        }
        let mut index = search(e, FN_00DF1260);
        while index >= 0 {
            e.call(FN_00DF2040, &args![index, count, flags, matrices]);
            let other = e.mem.u32(second);
            e.call(FN_00DF0810, &args![index, other, flags, rows]);
            index = search(e, FN_00DF1260);
        }
    });
    let mut previous = [[0.0f32; 4]; 2];
    for index in (0..count).rev() {
        let body_a = e.mem.u32(pairs + 4 * index).wrapping_add(bodies);
        let body_b = e.mem.u32(pairs + 4 * index + 4).wrapping_add(bodies);
        let matrix: [Vec4; 12] = std::array::from_fn(|k| {
            load_vec(e, matrices + 0x3c0 * index + 0x180 + 0x10 * k as u32)
        });
        let [p, q] = previous;
        let from_p = add(
            add(mul(matrix[1], splat(p[1])), mul(splat(p[0]), matrix[0])),
            mul(matrix[2], splat(p[2])),
        );
        let from_q = add(
            add(mul(matrix[4], splat(q[1])), mul(matrix[3], splat(q[0]))),
            mul(matrix[5], splat(q[2])),
        );
        let third = add(
            add(mul(matrix[7], splat(p[1])), mul(splat(p[0]), matrix[6])),
            mul(matrix[8], splat(p[2])),
        );
        let fourth = add(
            add(
                add(mul(matrix[10], splat(q[1])), mul(matrix[9], splat(q[0]))),
                mul(matrix[11], splat(q[2])),
            ),
            third,
        );
        let pair = vectors + 0x20 + 0x20 * index;
        let first = sub(load_vec(e, pair), add(from_q, from_p));
        let last = sub(load_vec(e, pair + 0x10), fourth);
        for component in 0..3u32 {
            let link_rows: [Vec4; 3] = std::array::from_fn(|k| {
                load_vec(e, links + 0x90 * index + 0x10 * (3 * component + k as u32))
            });
            apply_chain_component(
                e,
                body_a,
                body_b,
                link_rows,
                first[component as usize],
                true,
            );
            let slot = results + 24 * index + 4 * component;
            let total = e.mem.f32(slot) + first[component as usize];
            e.mem.set_f32(slot, total);
        }
        for component in 0..3u32 {
            let value = splat(last[component as usize]);
            let rows_a = load_vec(e, seconds + 0x60 * index + 0x20 * component);
            let moment_a = mul(value, load_vec(e, body_a + 0x30));
            let angular_a = add(mul(moment_a, rows_a), load_vec(e, body_a + 0x20));
            store_vec(e, body_a + 0x20, angular_a);
            let rows_b = load_vec(e, seconds + 0x60 * index + 0x20 * component + 0x10);
            let moment_b = mul(value, load_vec(e, body_b + 0x30));
            let angular_b = add(mul(moment_b, rows_b), load_vec(e, body_b + 0x20));
            store_vec(e, body_b + 0x20, angular_b);
            let slot = results + 24 * index + 12 + 4 * component;
            let total = e.mem.f32(slot) + last[component as usize];
            e.mem.set_f32(slot, total);
        }
        previous = [first, last];
    }
}

// ---------------------------------------------------------------------------------------------
// `hkSolveStepJacobians` (00d8aac0) and `hkSolveConstraintsByTheSteps` (00d8fb70).
//
// Both functions are one 2,600-instruction loop over the "schema" of a solver step: a list of
// records, each starting with a type byte, that is walked once with the exe's own stack frame,
// registers, SSE registers and x87 stack. They are written the way the exe runs: a state machine
// over the exe's basic blocks (the block is identified by the address of its first instruction)
// in which every instruction is one statement, in the exe's order, with its original text as a
// comment. This keeps every SSE lane operation, every x87 operation (a `float` load widens, the
// arithmetic is done in `double`, a `float` store rounds) and every flag test as the exe has it.
// ---------------------------------------------------------------------------------------------

/// The vector constant (`010c7160`) the clamping record types subtract the accumulated impulse
/// from (read through its first lane).
const VECTOR_CONSTANT_010C7160: u32 = 0x010c_7160;
/// `hkpProcessFlyingColors` (`00df22f0`, cdecl, one word: the flag to set), the Havok license
/// check `hkSolveConstraintsByTheSteps` runs first, and the flag byte it sets (`01269204`).
const LICENSE_CHECK: u32 = 0x00df_22f0;
const LICENSE_VALID_FLAG: u32 = 0x0126_9204;
/// The vector helpers the record types call (all with the vectors in memory): `005dbf20`
/// (thiscall, one float: stored in the first lane of `this`, the other lanes cleared; answers
/// `this`), `00c7f750` (`hkVector4::setMul3`, thiscall,
/// two words), `00c6ccc0` and `00c6cce0` (cdecl `(out, a, b)`: `a` with the first lane minus or
/// plus the first lane of `b`) and `00891060` (cdecl `(out, a, b)`: first lanes multiplied).
const VECTOR_FROM_FLOAT: u32 = 0x005d_bf20;
const SET_MUL3: u32 = 0x00c7_f750;
const VECTOR_SUBTRACT_FIRST_LANE: u32 = 0x00c6_ccc0;
const VECTOR_ADD_FIRST_LANE: u32 = 0x00c6_cce0;
const VECTOR_MULTIPLY_FIRST_LANE: u32 = 0x0089_1060;
/// The CRT square root (`00ec6040`, argument in `ST0`, answer in `ST0`).
const X87_SQUARE_ROOT: u32 = 0x00ec_6040;
/// The size of the stack block both functions run in (their frames need about 0x2c0 bytes).
const SOLVER_STACK_SIZE: u32 = 0x800;

/// `MAXPS`: per lane the destination when it is greater than the source, else the source (a NaN
/// or an equal pair gives the source).
fn max_ps(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| if x > y { x } else { y })
}

/// `MINPS`: per lane the destination when it is less than the source, else the source.
fn min_ps(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| if x < y { x } else { y })
}

/// `ADDSS`: the first lane is added, the other lanes of the destination stay.
fn add_ss(a: Vec4, b: Vec4) -> Vec4 {
    [a[0] + b[0], a[1], a[2], a[3]]
}

/// `SUBSS`.
fn sub_ss(a: Vec4, b: Vec4) -> Vec4 {
    [a[0] - b[0], a[1], a[2], a[3]]
}

/// `MULSS`.
fn mul_ss(a: Vec4, b: Vec4) -> Vec4 {
    [a[0] * b[0], a[1], a[2], a[3]]
}

/// `MOVSS xmm, dword ptr [..]`: the word in the first lane, the other lanes cleared.
fn movss_load(bits: u32) -> Vec4 {
    [f32::from_bits(bits), 0.0, 0.0, 0.0]
}

/// The x87 register stack (the last element is `ST0`). Values live in double precision; a
/// `float` load widens (a signalling NaN is quieted) and a `float` store rounds.
struct X87Stack {
    values: Vec<f64>,
}

impl X87Stack {
    fn depth(&self, index: usize) -> usize {
        self.values.len() - 1 - index
    }

    fn st(&self, index: usize) -> f64 {
        self.values[self.depth(index)]
    }

    fn set(&mut self, index: usize, value: f64) {
        let depth = self.depth(index);
        self.values[depth] = value;
    }

    fn push(&mut self, value: f64) {
        assert!(self.values.len() < 8, "x87 stack overflow");
        self.values.push(value);
    }

    fn pop(&mut self) -> f64 {
        self.values.pop().expect("x87 stack underflow")
    }

    /// `FLD ST(index)`.
    fn fld_st(&mut self, index: usize) {
        self.push(self.st(index));
    }

    /// `FSTP ST(index)` (index above 0): `ST(index) = ST0`, then pop.
    fn fstp_st(&mut self, index: usize) {
        self.set(index, self.st(0));
        self.pop();
    }

    /// `FXCH ST(index)`.
    fn fxch(&mut self, index: usize) {
        let (top, other) = (self.depth(0), self.depth(index));
        self.values.swap(top, other);
    }

    fn fchs(&mut self) {
        self.set(0, -self.st(0));
    }

    fn fabs(&mut self) {
        self.set(0, self.st(0).abs());
    }

    /// `FMUL ST0, ST(index)` (`D8 C8+i`).
    fn fmul_st0(&mut self, index: usize) {
        self.set(0, self.st(0) * self.st(index));
    }

    /// `FMUL ST(index), ST0` (`DC C8+i`).
    fn fmul_sti(&mut self, index: usize) {
        self.set(index, self.st(index) * self.st(0));
    }

    /// `FADDP ST(index), ST0` (`DE C0+i`).
    fn faddp(&mut self, index: usize) {
        self.set(index, self.st(index) + self.st(0));
        self.pop();
    }

    /// `FMULP ST(index), ST0` (`DE C8+i`).
    fn fmulp(&mut self, index: usize) {
        self.set(index, self.st(index) * self.st(0));
        self.pop();
    }

    /// `FSUBP ST(index), ST0` (`DE E8+i`): `ST(index) = ST(index) - ST0`.
    fn fsubp(&mut self, index: usize) {
        self.set(index, self.st(index) - self.st(0));
        self.pop();
    }

    /// `FDIVRP ST(index), ST0` (`DE F0+i`): `ST(index) = ST0 / ST(index)`.
    fn fdivrp(&mut self, index: usize) {
        self.set(index, self.st(0) / self.st(index));
        self.pop();
    }
}

/// The registers, SSE registers, x87 stack, status flags and stack of the two big solver
/// functions. `esp` and `ebp` are addresses in the stack block the function runs in; only the
/// flags the code tests (`zf`, `pf`, `cf`) and the x87 condition bits (`c0`, `c2`, `c3`) are
/// kept.
struct SolverMachine<'a> {
    e: &'a mut Engine,
    eax: u32,
    ecx: u32,
    edx: u32,
    ebx: u32,
    esp: u32,
    ebp: u32,
    esi: u32,
    edi: u32,
    xmm: [Vec4; 8],
    x87: X87Stack,
    c0: bool,
    c2: bool,
    c3: bool,
    zf: bool,
    pf: bool,
    cf: bool,
}

impl<'a> SolverMachine<'a> {
    /// A machine about to run a cdecl function whose `arguments` were pushed on the stack
    /// block that starts at `stack` (with a return address of 0 below them).
    fn new(e: &'a mut Engine, stack: u32, arguments: &[u32]) -> SolverMachine<'a> {
        let entry_esp = stack + SOLVER_STACK_SIZE - 0x40;
        for (index, word) in arguments.iter().enumerate() {
            e.mem.set_u32(entry_esp + 4 + 4 * index as u32, *word);
        }
        SolverMachine {
            e,
            eax: 0,
            ecx: 0,
            edx: 0,
            ebx: 0,
            esp: entry_esp,
            ebp: 0,
            esi: 0,
            edi: 0,
            xmm: [[0.0; 4]; 8],
            x87: X87Stack { values: Vec::new() },
            c0: false,
            c2: false,
            c3: false,
            zf: false,
            pf: false,
            cf: false,
        }
    }

    fn push(&mut self, value: u32) {
        self.esp = self.esp.wrapping_sub(4);
        self.e.mem.set_u32(self.esp, value);
    }

    fn pop(&mut self) -> u32 {
        let value = self.e.mem.u32(self.esp);
        self.esp = self.esp.wrapping_add(4);
        value
    }

    /// The zero, parity and sign-independent flags every ALU instruction leaves for `result`
    /// (of `bits` bits): the parity flag is the parity of the low byte.
    fn result_flags(&mut self, result: u32, bits: u32) {
        let mask = if bits == 32 {
            u32::MAX
        } else {
            (1 << bits) - 1
        };
        self.zf = result & mask == 0;
        self.pf = (result as u8).count_ones() % 2 == 0;
    }

    fn add32(&mut self, a: u32, b: u32) -> u32 {
        let (result, carry) = a.overflowing_add(b);
        self.cf = carry;
        self.result_flags(result, 32);
        result
    }

    fn sub32(&mut self, a: u32, b: u32) -> u32 {
        let (result, borrow) = a.overflowing_sub(b);
        self.cf = borrow;
        self.result_flags(result, 32);
        result
    }

    fn and32(&mut self, a: u32, b: u32) -> u32 {
        let result = a & b;
        self.cf = false;
        self.result_flags(result, 32);
        result
    }

    fn xor32(&mut self, a: u32, b: u32) -> u32 {
        let result = a ^ b;
        self.cf = false;
        self.result_flags(result, 32);
        result
    }

    fn neg32(&mut self, a: u32) -> u32 {
        let result = a.wrapping_neg();
        self.cf = a != 0;
        self.result_flags(result, 32);
        result
    }

    /// `SHL` by `count` (1 to 31): the carry is the last bit shifted out.
    fn shl32(&mut self, a: u32, count: u32) -> u32 {
        let result = a << count;
        self.cf = (a >> (32 - count)) & 1 != 0;
        self.result_flags(result, 32);
        result
    }

    fn cmp32(&mut self, a: u32, b: u32) {
        self.sub32(a, b);
    }

    fn cmp8(&mut self, a: u8, b: u8) {
        self.cf = a < b;
        self.result_flags(u32::from(a.wrapping_sub(b)), 8);
    }

    fn test8(&mut self, a: u8, b: u8) {
        self.cf = false;
        self.result_flags(u32::from(a & b), 8);
    }

    /// `FLD dword ptr`.
    fn fld32(&mut self, address: u32) {
        let value = f64::from(x87_quiet(self.e.mem.f32(address)));
        self.x87.push(value);
    }

    /// `FST dword ptr`.
    fn fst32(&mut self, address: u32) {
        let value = self.x87.st(0) as f32;
        self.e.mem.set_f32(address, value);
    }

    /// `FSTP dword ptr`.
    fn fstp32(&mut self, address: u32) {
        self.fst32(address);
        self.x87.pop();
    }

    fn float_operand(&self, address: u32) -> f64 {
        f64::from(x87_quiet(self.e.mem.f32(address)))
    }

    /// `FADD dword ptr`.
    fn fadd32(&mut self, address: u32) {
        let value = self.float_operand(address);
        self.x87.set(0, self.x87.st(0) + value);
    }

    /// `FSUB dword ptr`: `ST0 = ST0 - operand`.
    fn fsub32(&mut self, address: u32) {
        let value = self.float_operand(address);
        self.x87.set(0, self.x87.st(0) - value);
    }

    /// `FMUL dword ptr`.
    fn fmul32(&mut self, address: u32) {
        let value = self.float_operand(address);
        self.x87.set(0, self.x87.st(0) * value);
    }

    /// `FDIVR dword ptr`: `ST0 = operand / ST0`.
    fn fdivr32(&mut self, address: u32) {
        let value = self.float_operand(address);
        self.x87.set(0, value / self.x87.st(0));
    }

    /// `FIMUL dword ptr`: `ST0 = ST0 * operand` (a 32-bit integer).
    fn fimul32(&mut self, address: u32) {
        let value = f64::from(self.e.mem.i32(address));
        self.x87.set(0, self.x87.st(0) * value);
    }

    /// The condition bits an `FCOM` leaves: `C0` when `a` is below `b`, `C3` when they are
    /// equal, all three when they are unordered.
    fn compare(&mut self, a: f64, b: f64) {
        if a.is_nan() || b.is_nan() {
            (self.c0, self.c2, self.c3) = (true, true, true);
        } else {
            (self.c0, self.c2, self.c3) = (a < b, false, a == b);
        }
    }

    /// `FCOM ST(index)` and `FCOMP ST(index)`.
    fn fcom_st(&mut self, index: usize, pop: bool) {
        self.compare(self.x87.st(0), self.x87.st(index));
        if pop {
            self.x87.pop();
        }
    }

    /// `FCOMPP`.
    fn fcompp(&mut self) {
        self.fcom_st(1, true);
        self.x87.pop();
    }

    /// `FNSTSW AX`: `C0` is bit 8, `C2` bit 10 and `C3` bit 14 of the status word.
    fn fnstsw_ax(&mut self) {
        let status =
            (u32::from(self.c0) << 8) | (u32::from(self.c2) << 10) | (u32::from(self.c3) << 14);
        self.eax = (self.eax & 0xffff_0000) | status;
    }

    /// `CALL target` of a thiscall function with `words` stack words (`RET words * 4`): `ECX` is
    /// `this`, the callee pops its words and `EAX` gets the answer.
    fn call_this(&mut self, target: u32, words: u32) {
        let mut arguments = vec![self.ecx];
        for index in 0..words {
            arguments.push(self.e.mem.u32(self.esp.wrapping_add(4 * index)));
        }
        let result = self.e.call(target, &arguments);
        self.esp = self.esp.wrapping_add(4 * words);
        self.eax = result.eax;
    }

    /// `CALL target` of a cdecl function with `words` stack words (the caller pops them).
    fn call_cdecl(&mut self, target: u32, words: u32) {
        let arguments: Vec<u32> = (0..words)
            .map(|index| self.e.mem.u32(self.esp.wrapping_add(4 * index)))
            .collect();
        self.eax = self.e.call(target, &arguments).eax;
    }

    /// `CALL target` of a cdecl function of three words (`out`, `a`, `b`) that returns `out` in
    /// `EAX` (`MOV EAX, [EBP + 8]` in `00c6ccc0` and `00c6cce0`; `00891060` answers its first
    /// word as well). The registered translations of the three do not return it.
    fn call_cdecl_returning_first(&mut self, target: u32, words: u32) {
        let first = self.e.mem.u32(self.esp);
        self.call_cdecl(target, words);
        self.eax = first;
    }

    /// `CALL 00ec6040` (`_CIsqrt`): the argument is `ST0`, the answer replaces it.
    fn call_square_root(&mut self) {
        let argument = self.x87.pop();
        let result = self.e.call(X87_SQUARE_ROOT, &args![argument]);
        self.x87.push(result.st0);
    }

    /// `CALL 00d921c0` (cdecl `(destination, source)`): the exe's function copies a byte and
    /// leaves `destination` in `EAX` and the byte in `CL`.
    fn call_copy_byte(&mut self) {
        let destination = self.e.mem.u32(self.esp);
        let source = self.e.mem.u32(self.esp.wrapping_add(4));
        fn_00d921c0(self.e, destination, source);
        self.eax = destination;
        self.ecx = (self.ecx & 0xffff_ff00) | u32::from(self.e.mem.u8(destination));
    }

    /// `CALL 00d921d0` (cdecl `(source)`): answers the table byte in `EAX` and leaves the
    /// sign-extended byte at `source` in `ECX`.
    fn call_table_byte(&mut self) {
        let source = self.e.mem.u32(self.esp);
        self.eax = u32::from(fn_00d921d0(self.e, source));
        self.ecx = i32::from(self.e.mem.i8(source)) as u32;
    }

    /// The five stack words of `CALL 00d893a0`, `00d89690` and `00d8a3a0` (cdecl).
    fn chain_arguments(&self) -> [u32; 5] {
        std::array::from_fn(|index| self.e.mem.u32(self.esp.wrapping_add(4 * index as u32)))
    }

    /// `CALL 00d893a0` (`hkSolver_solveStiffSpringChain`).
    fn call_stiff_spring_chain(&mut self) {
        let [a, b, c, d, f] = self.chain_arguments();
        hk_solver_solve_stiff_spring_chain(self.e, a, b, c, d, f);
    }

    /// `CALL 00d89690` (`hkSolver_solveBallAndSocketChain`).
    fn call_ball_and_socket_chain(&mut self) {
        let [a, b, c, d, f] = self.chain_arguments();
        hk_solver_solve_ball_and_socket_chain(self.e, a, b, c, d, f);
    }

    /// `CALL 00d8a3a0` (`hkSolver_solvePoweredChain`).
    fn call_powered_chain(&mut self) {
        let [a, b, c, d, f] = self.chain_arguments();
        hk_solver_solve_powered_chain(self.e, a, b, c, d, f);
    }
}
/// The switch table at `00d8cfa8`: the block each record type jumps to.
const STEP_JACOBIANS_SWITCH: [u32; 32] = [
    0x00d8_cf9e,
    0x00d8_aaf0,
    0x00d8_cf69,
    0x00d8_ab09,
    0x00d8_cf9d,
    0x00d8_c1a0,
    0x00d8_c2b2,
    0x00d8_c3e0,
    0x00d8_c794,
    0x00d8_c643,
    0x00d8_c96d,
    0x00d8_cc51,
    0x00d8_c0f0,
    0x00d8_bcb1,
    0x00d8_bbaf,
    0x00d8_becd,
    0x00d8_b071,
    0x00d8_b071,
    0x00d8_b18e,
    0x00d8_b18e,
    0x00d8_ab1f,
    0x00d8_ab1f,
    0x00d8_b942,
    0x00d8_b5f0,
    0x00d8_b421,
    0x00d8_cde0,
    0x00d8_cdfb,
    0x00d8_ce72,
    0x00d8_cf9d,
    0x00d8_cf1c,
    0x00d8_cf45,
    0x00d8_cf71,
];

impl SolverMachine<'_> {
    /// The blocks of the function that starts at `0x00d8_aac0`, one arm per block.
    fn run_step_jacobians(&mut self) -> u32 {
        let mut block: u32 = 0x00d8_aac0;
        loop {
            match block {
                0x00d8_aac0 => {
                    self.push(self.ebp); // PUSH EBP
                    self.ebp = self.esp; // MOV EBP,ESP
                    self.esp = self.and32(self.esp, 0xfffffff0); // AND ESP,0xfffffff0
                    self.esp = self.sub32(self.esp, 0x284); // SUB ESP,0x284
                    self.x87.push(0.0); // FLDZ
                    self.push(self.ebx); // PUSH EBX
                    self.push(self.esi); // PUSH ESI
                    self.fstp32(self.esp.wrapping_add(0x74)); // FSTP float ptr [ESP + 0x74]
                    self.esi = self.e.mem.u32(self.ebp.wrapping_add(0xc)); // MOV ESI,dword ptr [EBP + 0xc]
                    self.cmp8(self.e.mem.u8(self.esi), 0x0); // CMP byte ptr [ESI],0x0
                    self.push(self.edi); // PUSH EDI
                    self.edi = self.e.mem.u32(self.ebp.wrapping_add(0x14)); // MOV EDI,dword ptr [EBP + 0x14]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x74), 0x0); // MOV dword ptr [ESP + 0x74],0x0
                    if self.zf {
                        // JZ 0x00d8cf9e
                        block = 0x00d8_cf9e;
                        continue;
                    }
                    block = 0x00d8_aaec;
                }
                0x00d8_aaec => {
                    // LEA ESP,[ESP]: no operation
                    block = 0x00d8_aaf0;
                }
                0x00d8_aaf0 => {
                    self.ebx = u32::from(self.e.mem.u16(self.esi.wrapping_add(0x4))); // MOVZX EBX,word ptr [ESI + 0x4]
                    self.eax = u32::from(self.e.mem.u16(self.esi.wrapping_add(0x6))); // MOVZX EAX,word ptr [ESI + 0x6]
                    self.ecx = self.e.mem.u32(self.ebp.wrapping_add(0x10)); // MOV ECX,dword ptr [EBP + 0x10]
                    self.ebx = self.shl32(self.ebx, 7); // SHL EBX,0x7
                    self.eax = self.shl32(self.eax, 7); // SHL EAX,0x7
                    self.ebx = self.add32(self.ebx, self.ecx); // ADD EBX,ECX
                    self.eax = self.add32(self.eax, self.ecx); // ADD EAX,ECX
                    self.e.mem.set_u32(self.esp.wrapping_add(0x4c), self.eax); // MOV dword ptr [ESP + 0x4c],EAX
                    block = 0x00d8_ab09;
                }
                0x00d8_ab09 => {
                    self.esi = self.add32(self.esi, 0x10); // ADD ESI,0x10
                    block = 0x00d8_ab0c;
                }
                0x00d8_ab0c => {
                    self.eax = i32::from(self.e.mem.i8(self.esi)) as u32; // MOVSX EAX,byte ptr [ESI]
                    self.cmp32(self.eax, 0x1f); // CMP EAX,0x1f
                    if !self.cf && !self.zf {
                        // JA 0x00d8cf9d
                        block = 0x00d8_cf9d;
                        continue;
                    }
                    block = 0x00d8_ab18;
                }
                0x00d8_ab18 => {
                    block = STEP_JACOBIANS_SWITCH[self.eax as usize]; // JMP dword ptr [EAX*0x4 + 0xd8cfa8]
                }
                0x00d8_ab1f => {
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EDX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = load_vec(self.e, self.edx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [EDX + 0x10]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = load_vec(self.e, self.edx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [EDX + 0x20]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi); // MOVAPS XMM2,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.ecx = self.edx.wrapping_add(0x10); // LEA ECX,[EDX + 0x10]
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.edx = self.add32(self.edx, 0x20); // ADD EDX,0x20
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xcc), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0xcc],XMM2
                    self.fld32(self.esp.wrapping_add(0xcc)); // FLD float ptr [ESP + 0xcc]
                    self.fst32(self.esp.wrapping_add(0x1c0)); // FST float ptr [ESP + 0x1c0]
                    self.xmm[0] = load_vec(self.e, self.ecx); // MOVAPS XMM0,xmmword ptr [ECX]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x40)); // MOVAPS XMM3,xmmword ptr [ESI + 0x40]
                    self.xmm[2] = load_vec(self.e, self.edx); // MOVAPS XMM2,xmmword ptr [EDX]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x50)); // MOVAPS XMM0,xmmword ptr [ESI + 0x50]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [ESI + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xb8), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0xb8],XMM2
                    self.fld32(self.esp.wrapping_add(0xb8)); // FLD float ptr [ESP + 0xb8]
                    self.fst32(self.esp.wrapping_add(0x1c4)); // FST float ptr [ESP + 0x1c4]
                    self.fld32(self.esi.wrapping_add(0x2c)); // FLD float ptr [ESI + 0x2c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.x87.fld_st(1); // FLD ST1
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.x87.fld_st(1); // FLD ST1
                    self.fmul32(self.esi.wrapping_add(0x60)); // FMUL float ptr [ESI + 0x60]
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.fld32(self.esi.wrapping_add(0x5c)); // FLD float ptr [ESI + 0x5c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.fld32(self.esi.wrapping_add(0x60)); // FLD float ptr [ESI + 0x60]
                    self.x87.fmulp(3); // FMULP ST3
                    self.x87.faddp(2); // FADDP ST2,ST0
                    self.x87.fxch(1); // FXCH
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x30)); // FLD float ptr [ESP + 0x30]
                    self.fld32(self.edi); // FLD float ptr [EDI]
                    self.x87.fchs(); // FCHS
                    self.fcompp(); // FCOMPP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d8ad5e
                        block = 0x00d8_ad5e;
                        continue;
                    }
                    block = 0x00d8_ac2d;
                }
                0x00d8_ac2d => {
                    self.x87.pop(); // FSTP ST0
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fld32(self.edi.wrapping_add(0x4)); // FLD float ptr [EDI + 0x4]
                    self.x87.fchs(); // FCHS
                    self.fcompp(); // FCOMPP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d8ad7b
                        block = 0x00d8_ad7b;
                        continue;
                    }
                    block = 0x00d8_ac45;
                }
                0x00d8_ac45 => {
                    self.xmm[5] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x30))); // MOVSS XMM5,dword ptr [ESP + 0x30]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.xmm[6] = load_vec(self.e, self.esi); // MOVAPS XMM6,xmmword ptr [ESI]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.eax = self.add32(self.eax, 0x30); // ADD EAX,0x30
                    self.xmm[1] = self.xmm[5]; // MOVAPS XMM1,XMM5
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[5], 0x0); // SHUFPS XMM1,XMM5,0x0
                    self.xmm[0] = self.xmm[1]; // MOVAPS XMM0,XMM1
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax); // MOVAPS XMM2,xmmword ptr [EAX]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[6]); // MULPS XMM2,XMM6
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[6]); // MULPS XMM3,XMM6
                    self.xmm[6] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM6,xmmword ptr [EBX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[6]); // ADDPS XMM2,XMM6
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx); // MOVAPS XMM2,xmmword ptr [ECX]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx, self.xmm[2]); // MOVAPS xmmword ptr [ECX],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ESI + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x20],XMM0
                    self.xmm[0] = load_vec(self.e, self.edx); // MOVAPS XMM0,xmmword ptr [EDX]
                    self.xmm[1] = add(self.xmm[1], self.xmm[0]); // ADDPS XMM1,XMM0
                    store_vec(self.e, self.edx, self.xmm[1]); // MOVAPS xmmword ptr [EDX],XMM1
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[5]); // ADDSS XMM2,XMM5
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x50), self.eax); // MOV dword ptr [ESP + 0x50],EAX
                    self.eax = self.edi.wrapping_add(0x4); // LEA EAX,[EDI + 0x4]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.eax); // MOV dword ptr [ESP + 0x30],EAX
                    self.eax = self.esi.wrapping_add(0x30); // LEA EAX,[ESI + 0x30]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x10), self.eax); // MOV dword ptr [ESP + 0x10],EAX
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x50)); // MOV EAX,dword ptr [ESP + 0x50]
                    self.xmm[0] = self.xmm[4]; // MOVAPS XMM0,XMM4
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[4], 0x0); // SHUFPS XMM0,XMM4,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax); // MOVAPS XMM2,xmmword ptr [EAX]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x10)); // MOV EAX,dword ptr [ESP + 0x10]
                    self.xmm[5] = load_vec(self.e, self.eax); // MOVAPS XMM5,xmmword ptr [EAX]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[1], 0xff); // SHUFPS XMM2,XMM1,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[0], 0xff); // SHUFPS XMM3,XMM0,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[5]); // ADDPS XMM2,XMM5
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx); // MOVAPS XMM2,xmmword ptr [ECX]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx, self.xmm[2]); // MOVAPS xmmword ptr [ECX],XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[1]); // MULPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EAX,dword ptr [ESP + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.eax)); // MOVSS XMM0,dword ptr [EAX]
                    self.xmm[2] = add(self.xmm[2], self.xmm[3]); // ADDPS XMM2,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x20],XMM2
                    self.xmm[2] = load_vec(self.e, self.edx); // MOVAPS XMM2,xmmword ptr [EDX]
                    self.xmm[1] = add(self.xmm[1], self.xmm[2]); // ADDPS XMM1,XMM2
                    store_vec(self.e, self.edx, self.xmm[1]); // MOVAPS xmmword ptr [EDX],XMM1
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[4]); // ADDSS XMM0,XMM4
                    self.e.mem.set_u32(self.eax, self.xmm[0][0].to_bits()); // MOVSS dword ptr [EAX],XMM0
                    block = 0x00d8_aec0; // JMP 0x00d8aec0
                }
                0x00d8_ad5e => {
                    self.fmul32(self.esi.wrapping_add(0x4c)); // FMUL float ptr [ESI + 0x4c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fld32(self.edi.wrapping_add(0x4)); // FLD float ptr [EDI + 0x4]
                    self.x87.fchs(); // FCHS
                    self.fcompp(); // FCOMPP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x1); // TEST AH,0x1
                    if !self.zf {
                        // JNZ 0x00d8ae28
                        block = 0x00d8_ae28;
                        continue;
                    }
                    block = 0x00d8_ad7b;
                }
                0x00d8_ad7b => {
                    self.xmm[1] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x1c0))); // MOVSS XMM1,dword ptr [ESP + 0x1c0]
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EBX + 0x30]
                    self.xmm[4] = load_vec(self.e, self.esi); // MOVAPS XMM4,xmmword ptr [ESI]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xff); // SHUFPS XMM0,XMM0,0xff
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, VECTOR_CONSTANT_010C7160); // MOVAPS XMM1,xmmword ptr [0x010c7160]
                    self.xmm[2] = mul_ss(self.xmm[2], self.xmm[0]); // MULSS XMM2,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM0,dword ptr [EDI]
                    self.eax = self.add32(self.eax, 0x30); // ADD EAX,0x30
                    self.e.mem.set_u32(self.esp.wrapping_add(0x50), self.eax); // MOV dword ptr [ESP + 0x50],EAX
                    self.xmm[1] = sub_ss(self.xmm[1], self.xmm[0]); // SUBSS XMM1,XMM0
                    self.xmm[1] = max_ps(self.xmm[1], self.xmm[2]); // MAXPS XMM1,XMM2
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[1], 0x0); // SHUFPS XMM2,XMM1,0x0
                    self.xmm[0] = self.xmm[2]; // MOVAPS XMM0,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.eax); // MOVAPS XMM3,xmmword ptr [EAX]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[3]); // MULPS XMM2,XMM3
                    self.xmm[5] = self.xmm[0]; // MOVAPS XMM5,XMM0
                    self.xmm[5] = shufps(self.xmm[5], self.xmm[0], 0xff); // SHUFPS XMM5,XMM0,0xff
                    self.xmm[3] = self.xmm[4]; // MOVAPS XMM3,XMM4
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = add(self.xmm[3], self.xmm[5]); // ADDPS XMM3,XMM5
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[3]); // MOVAPS xmmword ptr [EBX + 0x10],XMM3
                    self.xmm[3] = load_vec(self.e, self.ecx); // MOVAPS XMM3,xmmword ptr [ECX]
                    self.xmm[6] = self.xmm[2]; // MOVAPS XMM6,XMM2
                    self.xmm[6] = shufps(self.xmm[6], self.xmm[2], 0xff); // SHUFPS XMM6,XMM2,0xff
                    self.xmm[4] = mul(self.xmm[4], self.xmm[6]); // MULPS XMM4,XMM6
                    self.xmm[3] = sub(self.xmm[3], self.xmm[4]); // SUBPS XMM3,XMM4
                    store_vec(self.e, self.ecx, self.xmm[3]); // MOVAPS xmmword ptr [ECX],XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [ESI + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[2]); // MULPS XMM3,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.xmm[0] = add(self.xmm[0], self.xmm[4]); // ADDPS XMM0,XMM4
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x20],XMM0
                    self.xmm[0] = load_vec(self.e, self.edx); // MOVAPS XMM0,xmmword ptr [EDX]
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    store_vec(self.e, self.edx, self.xmm[0]); // MOVAPS xmmword ptr [EDX],XMM0
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[1]); // ADDSS XMM2,XMM1
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    block = 0x00d8_aec0; // JMP 0x00d8aec0
                }
                0x00d8_ae28 => {
                    self.xmm[3] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM3,dword ptr [ESP + 0x10]
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.eax = self.edi.wrapping_add(0x4); // LEA EAX,[EDI + 0x4]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.eax); // MOV dword ptr [ESP + 0x30],EAX
                    self.eax = self.esi.wrapping_add(0x30); // LEA EAX,[ESI + 0x30]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x10), self.eax); // MOV dword ptr [ESP + 0x10],EAX
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [EAX + 0x30]
                    self.eax = self.add32(self.eax, 0x30); // ADD EAX,0x30
                    self.e.mem.set_u32(self.esp.wrapping_add(0x50), self.eax); // MOV dword ptr [ESP + 0x50],EAX
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x10)); // MOV EAX,dword ptr [ESP + 0x10]
                    self.xmm[2] = self.xmm[3]; // MOVAPS XMM2,XMM3
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[3], 0x0); // SHUFPS XMM2,XMM3,0x0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax); // MOVAPS XMM2,xmmword ptr [EAX]
                    self.xmm[6] = self.xmm[2]; // MOVAPS XMM6,XMM2
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[4] = shufps(self.xmm[4], self.xmm[0], 0xff); // SHUFPS XMM4,XMM0,0xff
                    self.xmm[6] = mul(self.xmm[6], self.xmm[4]); // MULPS XMM6,XMM4
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM4,xmmword ptr [EBX + 0x10]
                    self.xmm[4] = add(self.xmm[4], self.xmm[6]); // ADDPS XMM4,XMM6
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[4]); // MOVAPS xmmword ptr [EBX + 0x10],XMM4
                    self.xmm[4] = load_vec(self.e, self.ecx); // MOVAPS XMM4,xmmword ptr [ECX]
                    self.xmm[5] = self.xmm[1]; // MOVAPS XMM5,XMM1
                    self.xmm[5] = shufps(self.xmm[5], self.xmm[1], 0xff); // SHUFPS XMM5,XMM1,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[4] = sub(self.xmm[4], self.xmm[2]); // SUBPS XMM4,XMM2
                    store_vec(self.e, self.ecx, self.xmm[4]); // MOVAPS xmmword ptr [ECX],XMM4
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EAX + 0x20]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EAX,dword ptr [ESP + 0x30]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.eax)); // MOVSS XMM1,dword ptr [EAX]
                    self.xmm[4] = add(self.xmm[4], self.xmm[2]); // ADDPS XMM4,XMM2
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[4]); // MOVAPS xmmword ptr [EBX + 0x20],XMM4
                    self.xmm[2] = load_vec(self.e, self.edx); // MOVAPS XMM2,xmmword ptr [EDX]
                    self.xmm[2] = add(self.xmm[2], self.xmm[0]); // ADDPS XMM2,XMM0
                    store_vec(self.e, self.edx, self.xmm[2]); // MOVAPS xmmword ptr [EDX],XMM2
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[3]); // ADDSS XMM1,XMM3
                    self.e.mem.set_u32(self.eax, self.xmm[1][0].to_bits()); // MOVSS dword ptr [EAX],XMM1
                    block = 0x00d8_aec0;
                }
                0x00d8_aec0 => {
                    self.esi = self.add32(self.esi, 0x70); // ADD ESI,0x70
                    self.edi = self.add32(self.edi, 0x8); // ADD EDI,0x8
                    self.cmp8(self.e.mem.u8(self.esi), 0x14); // CMP byte ptr [ESI],0x14
                    if !self.zf {
                        // JNZ 0x00d8ab0c
                        block = 0x00d8_ab0c;
                        continue;
                    }
                    block = 0x00d8_aecf;
                }
                0x00d8_aecf => {
                    self.xmm[0] = load_vec(self.e, self.ecx); // MOVAPS XMM0,xmmword ptr [ECX]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = load_vec(self.e, self.edx); // MOVAPS XMM2,xmmword ptr [EDX]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi); // MOVAPS XMM2,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xc8), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0xc8],XMM2
                    self.fld32(self.esp.wrapping_add(0xc8)); // FLD float ptr [ESP + 0xc8]
                    self.fst32(self.esp.wrapping_add(0x1a0)); // FST float ptr [ESP + 0x1a0]
                    self.xmm[0] = load_vec(self.e, self.ecx); // MOVAPS XMM0,xmmword ptr [ECX]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x40)); // MOVAPS XMM3,xmmword ptr [ESI + 0x40]
                    self.xmm[1] = load_vec(self.e, self.edx); // MOVAPS XMM1,xmmword ptr [EDX]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[0]); // SUBPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x50)); // MOVAPS XMM0,xmmword ptr [ESI + 0x50]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [ESI + 0x30]
                    self.xmm[4] = self.xmm[1]; // MOVAPS XMM4,XMM1
                    self.xmm[4] = mul(self.xmm[4], self.xmm[2]); // MULPS XMM4,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[4]); // ADDPS XMM0,XMM4
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0x55); // SHUFPS XMM2,XMM0,0x55
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[0]); // ADDSS XMM2,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[2]); // ADDSS XMM0,XMM2
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0xff); // SHUFPS XMM1,XMM1,0xff
                    self.xmm[1] = sub_ss(self.xmm[1], self.xmm[0]); // SUBSS XMM1,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x84), self.xmm[1][0].to_bits()); // MOVSS dword ptr [ESP + 0x84],XMM1
                    self.fld32(self.esp.wrapping_add(0x84)); // FLD float ptr [ESP + 0x84]
                    self.fst32(self.esp.wrapping_add(0x1a4)); // FST float ptr [ESP + 0x1a4]
                    self.fld32(self.esi.wrapping_add(0x2c)); // FLD float ptr [ESI + 0x2c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esi.wrapping_add(0x60)); // FLD float ptr [ESI + 0x60]
                    self.x87.fmul_st0(1); // FMUL ST1
                    self.x87.fld_st(2); // FLD ST2
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.fld32(self.esi.wrapping_add(0x5c)); // FLD float ptr [ESI + 0x5c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esi.wrapping_add(0x60)); // FLD float ptr [ESI + 0x60]
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.x87.faddp(2); // FADDP ST2,ST0
                    self.x87.fxch(1); // FXCH
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x30)); // FLD float ptr [ESP + 0x30]
                    self.fld32(self.edi); // FLD float ptr [EDI]
                    self.x87.fchs(); // FCHS
                    self.fcompp(); // FCOMPP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d8b2c0
                        block = 0x00d8_b2c0;
                        continue;
                    }
                    block = 0x00d8_afd4;
                }
                0x00d8_afd4 => {
                    self.x87.pop(); // FSTP ST0
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fld32(self.edi.wrapping_add(0x4)); // FLD float ptr [EDI + 0x4]
                    self.x87.fchs(); // FCHS
                    self.fcompp(); // FCOMPP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d8b2dd
                        block = 0x00d8_b2dd;
                        continue;
                    }
                    block = 0x00d8_afec;
                }
                0x00d8_afec => {
                    self.xmm[5] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x30))); // MOVSS XMM5,dword ptr [ESP + 0x30]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[6] = load_vec(self.e, self.esi); // MOVAPS XMM6,xmmword ptr [ESI]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x50)); // MOV EAX,dword ptr [ESP + 0x50]
                    self.xmm[1] = load_vec(self.e, self.eax); // MOVAPS XMM1,xmmword ptr [EAX]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[2] = self.xmm[5]; // MOVAPS XMM2,XMM5
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[5], 0x0); // SHUFPS XMM2,XMM5,0x0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[6]); // MULPS XMM2,XMM6
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[6]); // MULPS XMM3,XMM6
                    self.xmm[6] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM6,xmmword ptr [EBX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[6]); // ADDPS XMM2,XMM6
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx); // MOVAPS XMM2,xmmword ptr [ECX]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx, self.xmm[2]); // MOVAPS xmmword ptr [ECX],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ESI + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[1]); // MULPS XMM2,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM1,dword ptr [EDI]
                    self.xmm[3] = add(self.xmm[3], self.xmm[0]); // ADDPS XMM3,XMM0
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[3]); // MOVAPS xmmword ptr [EBX + 0x20],XMM3
                    self.xmm[0] = load_vec(self.e, self.edx); // MOVAPS XMM0,xmmword ptr [EDX]
                    self.xmm[2] = add(self.xmm[2], self.xmm[0]); // ADDPS XMM2,XMM0
                    store_vec(self.e, self.edx, self.xmm[2]); // MOVAPS xmmword ptr [EDX],XMM2
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[5]); // ADDSS XMM1,XMM5
                    self.e.mem.set_u32(self.edi, self.xmm[1][0].to_bits()); // MOVSS dword ptr [EDI],XMM1
                    block = 0x00d8_b386; // JMP 0x00d8b386
                }
                0x00d8_b071 => {
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.edx = self.ecx.wrapping_add(0x30); // LEA EDX,[ECX + 0x30]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x50), self.edx); // MOV dword ptr [ESP + 0x50],EDX
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EDX,dword ptr [ESP + 0x4c]
                    self.ecx = self.add32(self.ecx, 0x10); // ADD ECX,0x10
                    self.edx = self.add32(self.edx, 0x20); // ADD EDX,0x20
                    self.eax = self.ebx.wrapping_add(0x10); // LEA EAX,[EBX + 0x10]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x10), self.ecx); // MOV dword ptr [ESP + 0x10],ECX
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.edx); // MOV dword ptr [ESP + 0x30],EDX
                    block = 0x00d8_b091;
                }
                0x00d8_b091 => {
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x10)); // MOV EDX,dword ptr [ESP + 0x10]
                    self.xmm[0] = load_vec(self.e, self.edx); // MOVAPS XMM0,xmmword ptr [EDX]
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EDX,dword ptr [ESP + 0x30]
                    self.xmm[3] = load_vec(self.e, self.edx); // MOVAPS XMM3,xmmword ptr [EDX]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi); // MOVAPS XMM3,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x50)); // MOV EDX,dword ptr [ESP + 0x50]
                    self.xmm[4] = mul(self.xmm[4], self.xmm[2]); // MULPS XMM4,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[4]); // ADDPS XMM0,XMM4
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[3], 0xff); // SHUFPS XMM3,XMM3,0xff
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, VECTOR_CONSTANT_010C7160); // MOVAPS XMM1,xmmword ptr [0x010c7160]
                    self.xmm[3] = sub_ss(self.xmm[3], self.xmm[0]); // SUBSS XMM3,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM0,dword ptr [EDI]
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[3] = mul_ss(self.xmm[3], self.xmm[2]); // MULSS XMM3,XMM2
                    self.xmm[2] = load_vec(self.e, self.edx); // MOVAPS XMM2,xmmword ptr [EDX]
                    self.edx = self.ebx.wrapping_add(0x20); // LEA EDX,[EBX + 0x20]
                    self.esi = self.add32(self.esi, 0x30); // ADD ESI,0x30
                    self.xmm[1] = sub_ss(self.xmm[1], self.xmm[0]); // SUBSS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[1] = max_ps(self.xmm[1], self.xmm[3]); // MAXPS XMM1,XMM3
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0x0); // SHUFPS XMM3,XMM1,0x0
                    self.xmm[2] = mul(self.xmm[2], self.xmm[3]); // MULPS XMM2,XMM3
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[0], 0xff); // SHUFPS XMM3,XMM0,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[4] = self.xmm[2]; // MOVAPS XMM4,XMM2
                    self.xmm[4] = shufps(self.xmm[4], self.xmm[2], 0xff); // SHUFPS XMM4,XMM2,0xff
                    self.xmm[4] = mul(self.xmm[4], self.xmm[5]); // MULPS XMM4,XMM5
                    self.xmm[5] = load_vec(self.e, self.eax); // MOVAPS XMM5,xmmword ptr [EAX]
                    self.xmm[5] = add(self.xmm[5], self.xmm[3]); // ADDPS XMM5,XMM3
                    store_vec(self.e, self.eax, self.xmm[5]); // MOVAPS xmmword ptr [EAX],XMM5
                    self.xmm[3] = load_vec(self.e, self.ecx); // MOVAPS XMM3,xmmword ptr [ECX]
                    self.xmm[3] = sub(self.xmm[3], self.xmm[4]); // SUBPS XMM3,XMM4
                    store_vec(self.e, self.ecx, self.xmm[3]); // MOVAPS xmmword ptr [ECX],XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_sub(0x20)); // MOVAPS XMM3,xmmword ptr [ESI + -0x20]
                    self.xmm[4] = load_vec(self.e, self.edx); // MOVAPS XMM4,xmmword ptr [EDX]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_sub(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + -0x10]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[2]); // MULPS XMM3,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.xmm[4] = add(self.xmm[4], self.xmm[0]); // ADDPS XMM4,XMM0
                    store_vec(self.e, self.edx, self.xmm[4]); // MOVAPS xmmword ptr [EDX],XMM4
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EDX,dword ptr [ESP + 0x30]
                    self.xmm[0] = load_vec(self.e, self.edx); // MOVAPS XMM0,xmmword ptr [EDX]
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    store_vec(self.e, self.edx, self.xmm[0]); // MOVAPS xmmword ptr [EDX],XMM0
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[1]); // ADDSS XMM2,XMM1
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    self.edx = i32::from(self.e.mem.i8(self.esi.wrapping_sub(0x30))) as u32; // MOVSX EDX,byte ptr [ESI + -0x30]
                    self.edx = u32::from(self.e.mem.u8(self.edx.wrapping_add(0x11b9434))); // MOVZX EDX,byte ptr [EDX + 0x11b9434]
                    self.edi = self.edi.wrapping_add(self.edx.wrapping_mul(4)); // LEA EDI,[EDI + EDX*0x4]
                    self.edx = (self.edx & 0xffffff00) | u32::from(self.e.mem.u8(self.esi)); // MOV DL,byte ptr [ESI]
                    self.cmp8(self.edx as u8, 0x10); // CMP DL,0x10
                    if self.zf {
                        // JZ 0x00d8b091
                        block = 0x00d8_b091;
                        continue;
                    }
                    block = 0x00d8_b189;
                }
                0x00d8_b189 => {
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_b18e => {
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EDX,dword ptr [ESP + 0x4c]
                    self.eax = self.ecx.wrapping_add(0x30); // LEA EAX,[ECX + 0x30]
                    self.ecx = self.add32(self.ecx, 0x10); // ADD ECX,0x10
                    self.edx = self.add32(self.edx, 0x20); // ADD EDX,0x20
                    self.e.mem.set_u32(self.esp.wrapping_add(0x50), self.eax); // MOV dword ptr [ESP + 0x50],EAX
                    self.eax = self.ebx.wrapping_add(0x10); // LEA EAX,[EBX + 0x10]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x10), self.ecx); // MOV dword ptr [ESP + 0x10],ECX
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.edx); // MOV dword ptr [ESP + 0x30],EDX
                                                                               // MOV EDI,EDI: no operation
                    block = 0x00d8_b1b0;
                }
                0x00d8_b1b0 => {
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x10)); // MOV EDX,dword ptr [ESP + 0x10]
                    self.xmm[0] = load_vec(self.e, self.edx); // MOVAPS XMM0,xmmword ptr [EDX]
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EDX,dword ptr [ESP + 0x30]
                    self.xmm[3] = load_vec(self.e, self.edx); // MOVAPS XMM3,xmmword ptr [EDX]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi); // MOVAPS XMM3,xmmword ptr [ESI]
                    self.xmm[4] = mul(self.xmm[4], self.xmm[2]); // MULPS XMM4,XMM2
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x50)); // MOV EDX,dword ptr [ESP + 0x50]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[4]); // ADDPS XMM0,XMM4
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM1,dword ptr [EDI]
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[3], 0xff); // SHUFPS XMM3,XMM3,0xff
                    self.xmm[3] = sub_ss(self.xmm[3], self.xmm[0]); // SUBSS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[3] = mul_ss(self.xmm[3], self.xmm[2]); // MULSS XMM3,XMM2
                    self.xmm[2] = load_vec(self.e, VECTOR_CONSTANT_010C7160); // MOVAPS XMM2,xmmword ptr [0x010c7160]
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[1]); // SUBSS XMM2,XMM1
                    self.xmm[2] = max_ps(self.xmm[2], self.xmm[3]); // MAXPS XMM2,XMM3
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xff); // SHUFPS XMM0,XMM0,0xff
                    self.xmm[0] = sub_ss(self.xmm[0], self.xmm[1]); // SUBSS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.edx); // MOVAPS XMM1,xmmword ptr [EDX]
                    self.xmm[0] = min_ps(self.xmm[0], self.xmm[2]); // MINPS XMM0,XMM2
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[2] = self.xmm[4]; // MOVAPS XMM2,XMM4
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[4], 0x0); // SHUFPS XMM2,XMM4,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.eax); // MOVAPS XMM5,xmmword ptr [EAX]
                    self.xmm[2] = add(self.xmm[2], self.xmm[5]); // ADDPS XMM2,XMM5
                    store_vec(self.e, self.eax, self.xmm[2]); // MOVAPS xmmword ptr [EAX],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx); // MOVAPS XMM2,xmmword ptr [ECX]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx, self.xmm[2]); // MOVAPS xmmword ptr [ECX],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ESI + 0x20]
                    self.edx = self.ebx.wrapping_add(0x20); // LEA EDX,[EBX + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    store_vec(self.e, self.edx, self.xmm[0]); // MOVAPS xmmword ptr [EDX],XMM0
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EDX,dword ptr [ESP + 0x30]
                    self.xmm[0] = load_vec(self.e, self.edx); // MOVAPS XMM0,xmmword ptr [EDX]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.edx, self.xmm[0]); // MOVAPS xmmword ptr [EDX],XMM0
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[4]); // ADDSS XMM2,XMM4
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    self.edx = i32::from(self.e.mem.i8(self.esi)) as u32; // MOVSX EDX,byte ptr [ESI]
                    self.edx = u32::from(self.e.mem.u8(self.edx.wrapping_add(0x11b9434))); // MOVZX EDX,byte ptr [EDX + 0x11b9434]
                    self.edi = self.edi.wrapping_add(self.edx.wrapping_mul(4)); // LEA EDI,[EDI + EDX*0x4]
                    self.edx = (self.edx & 0xffffff00)
                        | u32::from(self.e.mem.u8(self.esi.wrapping_add(0x30))); // MOV DL,byte ptr [ESI + 0x30]
                    self.esi = self.add32(self.esi, 0x30); // ADD ESI,0x30
                    self.cmp8(self.edx as u8, 0x12); // CMP DL,0x12
                    if self.zf {
                        // JZ 0x00d8b1b0
                        block = 0x00d8_b1b0;
                        continue;
                    }
                    block = 0x00d8_b2bb;
                }
                0x00d8_b2bb => {
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_b2c0 => {
                    self.fmul32(self.esi.wrapping_add(0x4c)); // FMUL float ptr [ESI + 0x4c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fld32(self.edi.wrapping_add(0x4)); // FLD float ptr [EDI + 0x4]
                    self.x87.fchs(); // FCHS
                    self.fcompp(); // FCOMPP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x1); // TEST AH,0x1
                    if !self.zf {
                        // JNZ 0x00d8b37d
                        block = 0x00d8_b37d;
                        continue;
                    }
                    block = 0x00d8_b2dd;
                }
                0x00d8_b2dd => {
                    self.xmm[1] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x1a0))); // MOVSS XMM1,dword ptr [ESP + 0x1a0]
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x50)); // MOV EAX,dword ptr [ESP + 0x50]
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xff); // SHUFPS XMM0,XMM0,0xff
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, VECTOR_CONSTANT_010C7160); // MOVAPS XMM1,xmmword ptr [0x010c7160]
                    self.xmm[2] = mul_ss(self.xmm[2], self.xmm[0]); // MULSS XMM2,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM0,dword ptr [EDI]
                    self.xmm[1] = sub_ss(self.xmm[1], self.xmm[0]); // SUBSS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[1] = max_ps(self.xmm[1], self.xmm[2]); // MAXPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax); // MOVAPS XMM2,xmmword ptr [EAX]
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0x0); // SHUFPS XMM3,XMM1,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[2] = mul(self.xmm[2], self.xmm[3]); // MULPS XMM2,XMM3
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[0], 0xff); // SHUFPS XMM3,XMM0,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[4] = self.xmm[2]; // MOVAPS XMM4,XMM2
                    self.xmm[4] = shufps(self.xmm[4], self.xmm[2], 0xff); // SHUFPS XMM4,XMM2,0xff
                    self.xmm[4] = mul(self.xmm[4], self.xmm[5]); // MULPS XMM4,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[5] = add(self.xmm[5], self.xmm[3]); // ADDPS XMM5,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[5]); // MOVAPS xmmword ptr [EBX + 0x10],XMM5
                    self.xmm[3] = load_vec(self.e, self.ecx); // MOVAPS XMM3,xmmword ptr [ECX]
                    self.xmm[3] = sub(self.xmm[3], self.xmm[4]); // SUBPS XMM3,XMM4
                    store_vec(self.e, self.ecx, self.xmm[3]); // MOVAPS xmmword ptr [ECX],XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [ESI + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[2]); // MULPS XMM3,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.xmm[0] = add(self.xmm[0], self.xmm[4]); // ADDPS XMM0,XMM4
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x20],XMM0
                    self.xmm[0] = load_vec(self.e, self.edx); // MOVAPS XMM0,xmmword ptr [EDX]
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    store_vec(self.e, self.edx, self.xmm[0]); // MOVAPS xmmword ptr [EDX],XMM0
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[1]); // ADDSS XMM2,XMM1
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    block = 0x00d8_b40d; // JMP 0x00d8b40d
                }
                0x00d8_b37d => {
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    block = 0x00d8_b386;
                }
                0x00d8_b386 => {
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.eax = self.edi.wrapping_add(0x4); // LEA EAX,[EDI + 0x4]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.eax); // MOV dword ptr [ESP + 0x30],EAX
                    self.eax = self.esi.wrapping_add(0x30); // LEA EAX,[ESI + 0x30]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x10), self.eax); // MOV dword ptr [ESP + 0x10],EAX
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x50)); // MOV EAX,dword ptr [ESP + 0x50]
                    self.xmm[1] = load_vec(self.e, self.eax); // MOVAPS XMM1,xmmword ptr [EAX]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x10)); // MOV EAX,dword ptr [ESP + 0x10]
                    self.xmm[5] = load_vec(self.e, self.eax); // MOVAPS XMM5,xmmword ptr [EAX]
                    self.xmm[2] = self.xmm[4]; // MOVAPS XMM2,XMM4
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[4], 0x0); // SHUFPS XMM2,XMM4,0x0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[5] = add(self.xmm[5], self.xmm[2]); // ADDPS XMM5,XMM2
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[5]); // MOVAPS xmmword ptr [EBX + 0x10],XMM5
                    self.xmm[2] = load_vec(self.e, self.ecx); // MOVAPS XMM2,xmmword ptr [ECX]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx, self.xmm[2]); // MOVAPS xmmword ptr [ECX],XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EAX + 0x20]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EAX,dword ptr [ESP + 0x30]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.eax)); // MOVSS XMM1,dword ptr [EAX]
                    self.xmm[3] = add(self.xmm[3], self.xmm[2]); // ADDPS XMM3,XMM2
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[3]); // MOVAPS xmmword ptr [EBX + 0x20],XMM3
                    self.xmm[2] = load_vec(self.e, self.edx); // MOVAPS XMM2,xmmword ptr [EDX]
                    self.xmm[0] = add(self.xmm[0], self.xmm[2]); // ADDPS XMM0,XMM2
                    store_vec(self.e, self.edx, self.xmm[0]); // MOVAPS xmmword ptr [EDX],XMM0
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[4]); // ADDSS XMM1,XMM4
                    self.e.mem.set_u32(self.eax, self.xmm[1][0].to_bits()); // MOVSS dword ptr [EAX],XMM1
                    block = 0x00d8_b40d;
                }
                0x00d8_b40d => {
                    self.esi = self.add32(self.esi, 0x70); // ADD ESI,0x70
                    self.edi = self.add32(self.edi, 0x8); // ADD EDI,0x8
                    self.cmp8(self.e.mem.u8(self.esi), 0x17); // CMP byte ptr [ESI],0x17
                    if !self.zf {
                        // JNZ 0x00d8ab0c
                        block = 0x00d8_ab0c;
                        continue;
                    }
                    block = 0x00d8_b41c;
                }
                0x00d8_b41c => {
                    block = 0x00d8_b5f0; // JMP 0x00d8b5f0
                }
                0x00d8_b421 => {
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EBX + 0x20]
                    self.fld32(self.esi.wrapping_add(0x40)); // FLD float ptr [ESI + 0x40]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.xmm[0] = load_vec(self.e, self.esi); // MOVAPS XMM0,xmmword ptr [ESI]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.x87.fmul_sti(0); // FMUL ST0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EAX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.fld32(self.esi.wrapping_add(0xc)); // FLD float ptr [ESI + 0xc]
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [ESI + 0x10]
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0xff); // SHUFPS XMM1,XMM1,0xff
                    self.xmm[1] = sub_ss(self.xmm[1], self.xmm[0]); // SUBSS XMM1,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xac), self.xmm[1][0].to_bits()); // MOVSS dword ptr [ESP + 0xac],XMM1
                    self.fmul32(self.esp.wrapping_add(0xac)); // FMUL float ptr [ESP + 0xac]
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [ESI + 0x30]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EAX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [ESI + 0x30]
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0xff); // SHUFPS XMM1,XMM1,0xff
                    self.xmm[1] = sub_ss(self.xmm[1], self.xmm[0]); // SUBSS XMM1,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x8c), self.xmm[1][0].to_bits()); // MOVSS dword ptr [ESP + 0x8c],XMM1
                    self.fld32(self.esi.wrapping_add(0x2c)); // FLD float ptr [ESI + 0x2c]
                    self.fmul32(self.esp.wrapping_add(0x8c)); // FMUL float ptr [ESP + 0x8c]
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.fld32(self.esp.wrapping_add(0x30)); // FLD float ptr [ESP + 0x30]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.x87.fmul_sti(0); // FMUL ST0
                    self.x87.fld_st(1); // FLD ST1
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x50)); // FSTP float ptr [ESP + 0x50]
                    self.fld32(self.esp.wrapping_add(0x50)); // FLD float ptr [ESP + 0x50]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fcom_st(1, false); // FCOM
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d8b52d
                        block = 0x00d8_b52d;
                        continue;
                    }
                    block = 0x00d8_b4f8;
                }
                0x00d8_b4f8 => {
                    self.x87.fdivrp(1); // FDIVRP
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.call_square_root(); // CALL 0x00ec6040
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x30)); // FMUL float ptr [ESP + 0x30]
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    block = 0x00d8_b533; // JMP 0x00d8b533
                }
                0x00d8_b52d => {
                    self.x87.fstp_st(1); // FSTP ST1
                    self.x87.pop(); // FSTP ST0
                    self.x87.push(1.0); // FLD1
                    block = 0x00d8_b533;
                }
                0x00d8_b533 => {
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.fstp32(self.edi.wrapping_add(0x8)); // FSTP float ptr [EDI + 0x8]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EBX + 0x30]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EAX + 0x30]
                    self.xmm[1] = self.xmm[2]; // MOVAPS XMM1,XMM2
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[2], 0x0); // SHUFPS XMM1,XMM2,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[3] = mul(self.xmm[3], self.xmm[1]); // MULPS XMM3,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi); // MOVAPS XMM1,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM3,dword ptr [EDI]
                    self.xmm[4] = add(self.xmm[4], self.xmm[1]); // ADDPS XMM4,XMM1
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[4]); // MOVAPS xmmword ptr [EBX + 0x20],XMM4
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.eax.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EAX + 0x20],XMM0
                    self.xmm[3] = add_ss(self.xmm[3], self.xmm[2]); // ADDSS XMM3,XMM2
                    self.e.mem.set_u32(self.edi, self.xmm[3][0].to_bits()); // MOVSS dword ptr [EDI],XMM3
                    self.xmm[3] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x30))); // MOVSS XMM3,dword ptr [ESP + 0x30]
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [EAX + 0x30]
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = self.xmm[3]; // MOVAPS XMM2,XMM3
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[3], 0x0); // SHUFPS XMM2,XMM3,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [ESI + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi.wrapping_add(0x4))); // MOVSS XMM2,dword ptr [EDI + 0x4]
                    self.xmm[0] = add(self.xmm[0], self.xmm[4]); // ADDPS XMM0,XMM4
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x20],XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EAX + 0x20]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.eax.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EAX + 0x20],XMM0
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[3]); // ADDSS XMM2,XMM3
                    self.push(self.esi); // PUSH ESI
                    self.e
                        .mem
                        .set_u32(self.edi.wrapping_add(0x4), self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI + 0x4],XMM2
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.eax = i32::from(self.e.mem.i8(self.esi.wrapping_add(0x50))) as u32; // MOVSX EAX,byte ptr [ESI + 0x50]
                    self.esp = self.add32(self.esp, 0x4); // ADD ESP,0x4
                    self.esi = self.add32(self.esi, 0x50); // ADD ESI,0x50
                    self.cmp32(self.eax, 0x17); // CMP EAX,0x17
                    if !self.zf {
                        // JNZ 0x00d8b939
                        block = 0x00d8_b939;
                        continue;
                    }
                    block = 0x00d8_b5f0;
                }
                0x00d8_b5f0 => {
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[4] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [ECX + 0x20]
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.xmm[2] = load_vec(self.e, self.esi); // MOVAPS XMM2,xmmword ptr [ESI]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[4]); // MULPS XMM0,XMM4
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x94), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0x94],XMM2
                    self.fld32(self.esp.wrapping_add(0x94)); // FLD float ptr [ESP + 0x94]
                    self.fst32(self.esp.wrapping_add(0x1b0)); // FST float ptr [ESP + 0x1b0]
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x40)); // MOVAPS XMM3,xmmword ptr [ESI + 0x40]
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ECX + 0x20]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x50)); // MOVAPS XMM0,xmmword ptr [ESI + 0x50]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [ESI + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xb4), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0xb4],XMM2
                    self.fld32(self.esp.wrapping_add(0xb4)); // FLD float ptr [ESP + 0xb4]
                    self.fst32(self.esp.wrapping_add(0x1b4)); // FST float ptr [ESP + 0x1b4]
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x60)); // MOVAPS XMM0,xmmword ptr [ESI + 0x60]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ECX + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x70)); // MOVAPS XMM0,xmmword ptr [ESI + 0x70]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x70)); // MOVAPS XMM1,xmmword ptr [ESI + 0x70]
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0xff); // SHUFPS XMM1,XMM1,0xff
                    self.xmm[1] = sub_ss(self.xmm[1], self.xmm[0]); // SUBSS XMM1,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x9c), self.xmm[1][0].to_bits()); // MOVSS dword ptr [ESP + 0x9c],XMM1
                    self.fld32(self.esp.wrapping_add(0x9c)); // FLD float ptr [ESP + 0x9c]
                    self.fst32(self.esp.wrapping_add(0x1b8)); // FST float ptr [ESP + 0x1b8]
                    self.fmul32(self.esi.wrapping_add(0x6c)); // FMUL float ptr [ESI + 0x6c]
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.fld32(self.esi.wrapping_add(0x2c)); // FLD float ptr [ESI + 0x2c]
                    self.x87.fmul_st0(2); // FMUL ST2
                    self.fld32(self.esi.wrapping_add(0x80)); // FLD float ptr [ESI + 0x80]
                    self.x87.fmul_st0(2); // FMUL ST2
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x50)); // FSTP float ptr [ESP + 0x50]
                    self.fmul32(self.esi.wrapping_add(0x5c)); // FMUL float ptr [ESI + 0x5c]
                    self.fld32(self.esi.wrapping_add(0x80)); // FLD float ptr [ESI + 0x80]
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x50)); // FLD float ptr [ESP + 0x50]
                    self.fld32(self.esp.wrapping_add(0x30)); // FLD float ptr [ESP + 0x30]
                    self.x87.fld_st(1); // FLD ST1
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.fld_st(2); // FLD ST2
                    self.x87.fmulp(3); // FMULP ST3
                    self.x87.fxch(1); // FXCH
                    self.x87.faddp(2); // FADDP ST2,ST0
                    self.x87.fmul_sti(0); // FMUL ST0
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esi.wrapping_add(0x84)); // FLD float ptr [ESI + 0x84]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.x87.fld_st(1); // FLD ST1
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.fxch(1); // FXCH
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fcom_st(1, true); // FCOMP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d8b7b9
                        block = 0x00d8_b7b9;
                        continue;
                    }
                    block = 0x00d8_b76e;
                }
                0x00d8_b76e => {
                    self.call_square_root(); // CALL 0x00ec6040
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.x87.push(1.0); // FLD1
                    self.x87.fdivrp(1); // FDIVRP
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fmul32(self.esi.wrapping_add(0x84)); // FMUL float ptr [ESI + 0x84]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x50)); // FMUL float ptr [ESP + 0x50]
                    self.fstp32(self.esp.wrapping_add(0x50)); // FSTP float ptr [ESP + 0x50]
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x30)); // FMUL float ptr [ESP + 0x30]
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    block = 0x00d8_b7bd; // JMP 0x00d8b7bd
                }
                0x00d8_b7b9 => {
                    self.x87.pop(); // FSTP ST0
                    self.x87.push(1.0); // FLD1
                    block = 0x00d8_b7bd;
                }
                0x00d8_b7bd => {
                    self.fstp32(self.edi.wrapping_add(0xc)); // FSTP float ptr [EDI + 0xc]
                    self.xmm[5] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x50))); // MOVSS XMM5,dword ptr [ESP + 0x50]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.fld32(self.esi.wrapping_add(0x88)); // FLD float ptr [ESI + 0x88]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.fmul32(self.esp.wrapping_add(0x30)); // FMUL float ptr [ESP + 0x30]
                    self.xmm[6] = load_vec(self.e, self.esi); // MOVAPS XMM6,xmmword ptr [ESI]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[1] = self.xmm[5]; // MOVAPS XMM1,XMM5
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[5], 0x0); // SHUFPS XMM1,XMM5,0x0
                    self.xmm[0] = self.xmm[1]; // MOVAPS XMM0,XMM1
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [ECX + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[6]); // MULPS XMM2,XMM6
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[6]); // MULPS XMM3,XMM6
                    self.xmm[6] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM6,xmmword ptr [EBX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[6]); // ADDPS XMM2,XMM6
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ECX + 0x10]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [ECX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ESI + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x20],XMM0
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ECX + 0x20]
                    self.xmm[1] = add(self.xmm[1], self.xmm[0]); // ADDPS XMM1,XMM0
                    store_vec(self.e, self.ecx.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [ECX + 0x20],XMM1
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[5]); // ADDSS XMM2,XMM5
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.xmm[5] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM5,xmmword ptr [ESI + 0x30]
                    self.xmm[0] = self.xmm[4]; // MOVAPS XMM0,XMM4
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[4], 0x0); // SHUFPS XMM0,XMM4,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [ECX + 0x30]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[1], 0xff); // SHUFPS XMM2,XMM1,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[0], 0xff); // SHUFPS XMM3,XMM0,0xff
                    self.eax = self.esi.wrapping_add(0x30); // LEA EAX,[ESI + 0x30]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[5]); // ADDPS XMM2,XMM5
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ECX + 0x10]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [ECX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[1]); // MULPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.edi.wrapping_add(0x4))); // MOVSS XMM0,dword ptr [EDI + 0x4]
                    self.xmm[2] = add(self.xmm[2], self.xmm[3]); // ADDPS XMM2,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x20],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ECX + 0x20]
                    self.xmm[1] = add(self.xmm[1], self.xmm[2]); // ADDPS XMM1,XMM2
                    store_vec(self.e, self.ecx.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [ECX + 0x20],XMM1
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[4]); // ADDSS XMM0,XMM4
                    self.e
                        .mem
                        .set_u32(self.edi.wrapping_add(0x4), self.xmm[0][0].to_bits()); // MOVSS dword ptr [EDI + 0x4],XMM0
                    self.xmm[3] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x30))); // MOVSS XMM3,dword ptr [ESP + 0x30]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.xmm[0] = self.xmm[3]; // MOVAPS XMM0,XMM3
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[3], 0x0); // SHUFPS XMM0,XMM3,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [ECX + 0x30]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x60)); // MOVAPS XMM2,xmmword ptr [ESI + 0x60]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x70)); // MOVAPS XMM2,xmmword ptr [ESI + 0x70]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.edi.wrapping_add(0x8))); // MOVSS XMM0,dword ptr [EDI + 0x8]
                    self.xmm[1] = add(self.xmm[1], self.xmm[4]); // ADDPS XMM1,XMM4
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [EBX + 0x20],XMM1
                    self.xmm[1] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [ECX + 0x20]
                    self.xmm[2] = add(self.xmm[2], self.xmm[1]); // ADDPS XMM2,XMM1
                    store_vec(self.e, self.ecx.wrapping_add(0x20), self.xmm[2]); // MOVAPS xmmword ptr [ECX + 0x20],XMM2
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[3]); // ADDSS XMM0,XMM3
                    self.e
                        .mem
                        .set_u32(self.edi.wrapping_add(0x8), self.xmm[0][0].to_bits()); // MOVSS dword ptr [EDI + 0x8],XMM0
                    self.eax = (self.eax & 0xffffff00)
                        | u32::from(self.e.mem.u8(self.esi.wrapping_add(0x90))); // MOV AL,byte ptr [ESI + 0x90]
                    self.edi = self.add32(self.edi, 0x10); // ADD EDI,0x10
                    self.esi = self.add32(self.esi, 0x90); // ADD ESI,0x90
                    self.cmp8(self.eax as u8, 0x1); // CMP AL,0x1
                    if !self.zf {
                        // JNZ 0x00d8ab0c
                        block = 0x00d8_ab0c;
                        continue;
                    }
                    block = 0x00d8_b934;
                }
                0x00d8_b934 => {
                    block = 0x00d8_aaf0; // JMP 0x00d8aaf0
                }
                0x00d8_b939 => {
                    self.cmp32(self.eax, 0x16); // CMP EAX,0x16
                    if !self.zf {
                        // JNZ 0x00d8ab0c
                        block = 0x00d8_ab0c;
                        continue;
                    }
                    block = 0x00d8_b942;
                }
                0x00d8_b942 => {
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[4] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [ECX + 0x20]
                    self.xmm[2] = load_vec(self.e, self.esi); // MOVAPS XMM2,xmmword ptr [ESI]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[4]); // MULPS XMM0,XMM4
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xc4), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0xc4],XMM2
                    self.fld32(self.esp.wrapping_add(0xc4)); // FLD float ptr [ESP + 0xc4]
                    self.fst32(self.esp.wrapping_add(0x250)); // FST float ptr [ESP + 0x250]
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x40)); // MOVAPS XMM3,xmmword ptr [ESI + 0x40]
                    self.xmm[1] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [ECX + 0x20]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[0]); // SUBPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x50)); // MOVAPS XMM0,xmmword ptr [ESI + 0x50]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [ESI + 0x30]
                    self.xmm[4] = self.xmm[1]; // MOVAPS XMM4,XMM1
                    self.xmm[4] = mul(self.xmm[4], self.xmm[2]); // MULPS XMM4,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[4]); // ADDPS XMM0,XMM4
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0x55); // SHUFPS XMM2,XMM0,0x55
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[0]); // ADDSS XMM2,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[2]); // ADDSS XMM0,XMM2
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0xff); // SHUFPS XMM1,XMM1,0xff
                    self.xmm[1] = sub_ss(self.xmm[1], self.xmm[0]); // SUBSS XMM1,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xa4), self.xmm[1][0].to_bits()); // MOVSS dword ptr [ESP + 0xa4],XMM1
                    self.fld32(self.esp.wrapping_add(0xa4)); // FLD float ptr [ESP + 0xa4]
                    self.fst32(self.esp.wrapping_add(0x254)); // FST float ptr [ESP + 0x254]
                    self.fld32(self.esi.wrapping_add(0x64)); // FLD float ptr [ESI + 0x64]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.x87.fmul_sti(0); // FMUL ST0
                    self.fstp32(self.esp.wrapping_add(0x50)); // FSTP float ptr [ESP + 0x50]
                    self.fld32(self.esi.wrapping_add(0x2c)); // FLD float ptr [ESI + 0x2c]
                    self.x87.fmul_st0(2); // FMUL ST2
                    self.x87.fld_st(1); // FLD ST1
                    self.fmul32(self.esi.wrapping_add(0x60)); // FMUL float ptr [ESI + 0x60]
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.fmul32(self.esi.wrapping_add(0x5c)); // FMUL float ptr [ESI + 0x5c]
                    self.fld32(self.esi.wrapping_add(0x60)); // FLD float ptr [ESI + 0x60]
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x30)); // FLD float ptr [ESP + 0x30]
                    self.x87.fmul_sti(0); // FMUL ST0
                    self.x87.fld_st(1); // FLD ST1
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x50)); // FLD float ptr [ESP + 0x50]
                    self.fcom_st(1, false); // FCOM
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d8ba95
                        block = 0x00d8_ba95;
                        continue;
                    }
                    block = 0x00d8_ba5c;
                }
                0x00d8_ba5c => {
                    self.x87.fdivrp(1); // FDIVRP
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.call_square_root(); // CALL 0x00ec6040
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x30)); // FMUL float ptr [ESP + 0x30]
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    block = 0x00d8_ba9b; // JMP 0x00d8ba9b
                }
                0x00d8_ba95 => {
                    self.x87.fstp_st(1); // FSTP ST1
                    self.x87.pop(); // FSTP ST0
                    self.x87.push(1.0); // FLD1
                    block = 0x00d8_ba9b;
                }
                0x00d8_ba9b => {
                    self.fstp32(self.edi.wrapping_add(0x8)); // FSTP float ptr [EDI + 0x8]
                    self.xmm[5] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x30))); // MOVSS XMM5,dword ptr [ESP + 0x30]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[1] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [ECX + 0x30]
                    self.xmm[6] = load_vec(self.e, self.esi); // MOVAPS XMM6,xmmword ptr [ESI]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[2] = self.xmm[5]; // MOVAPS XMM2,XMM5
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[5], 0x0); // SHUFPS XMM2,XMM5,0x0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[6]); // MULPS XMM2,XMM6
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[6]); // MULPS XMM3,XMM6
                    self.xmm[6] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM6,xmmword ptr [EBX + 0x10]
                    self.xmm[6] = add(self.xmm[6], self.xmm[2]); // ADDPS XMM6,XMM2
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[6]); // MOVAPS xmmword ptr [EBX + 0x10],XMM6
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ECX + 0x10]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [ECX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[3] = add(self.xmm[3], self.xmm[2]); // ADDPS XMM3,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM1,dword ptr [EDI]
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[3]); // MOVAPS xmmword ptr [EBX + 0x20],XMM3
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ECX + 0x20]
                    self.xmm[2] = add(self.xmm[2], self.xmm[0]); // ADDPS XMM2,XMM0
                    store_vec(self.e, self.ecx.wrapping_add(0x20), self.xmm[2]); // MOVAPS xmmword ptr [ECX + 0x20],XMM2
                    self.eax = self.esi.wrapping_add(0x30); // LEA EAX,[ESI + 0x30]
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[5]); // ADDSS XMM1,XMM5
                    self.e.mem.set_u32(self.edi, self.xmm[1][0].to_bits()); // MOVSS dword ptr [EDI],XMM1
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[1] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [ECX + 0x30]
                    self.xmm[2] = self.xmm[4]; // MOVAPS XMM2,XMM4
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[4], 0x0); // SHUFPS XMM2,XMM4,0x0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax); // MOVAPS XMM2,xmmword ptr [EAX]
                    self.xmm[6] = self.xmm[2]; // MOVAPS XMM6,XMM2
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[0], 0xff); // SHUFPS XMM3,XMM0,0xff
                    self.xmm[6] = mul(self.xmm[6], self.xmm[3]); // MULPS XMM6,XMM3
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = add(self.xmm[3], self.xmm[6]); // ADDPS XMM3,XMM6
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[3]); // MOVAPS xmmword ptr [EBX + 0x10],XMM3
                    self.xmm[3] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ECX + 0x10]
                    self.xmm[5] = self.xmm[1]; // MOVAPS XMM5,XMM1
                    self.xmm[5] = shufps(self.xmm[5], self.xmm[1], 0xff); // SHUFPS XMM5,XMM1,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = sub(self.xmm[3], self.xmm[2]); // SUBPS XMM3,XMM2
                    store_vec(self.e, self.ecx.wrapping_add(0x10), self.xmm[3]); // MOVAPS xmmword ptr [ECX + 0x10],XMM3
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EAX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.edi.wrapping_add(0x4))); // MOVSS XMM1,dword ptr [EDI + 0x4]
                    self.xmm[3] = add(self.xmm[3], self.xmm[2]); // ADDPS XMM3,XMM2
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[3]); // MOVAPS xmmword ptr [EBX + 0x20],XMM3
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ECX + 0x20]
                    self.xmm[2] = add(self.xmm[2], self.xmm[0]); // ADDPS XMM2,XMM0
                    store_vec(self.e, self.ecx.wrapping_add(0x20), self.xmm[2]); // MOVAPS xmmword ptr [ECX + 0x20],XMM2
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[4]); // ADDSS XMM1,XMM4
                    self.e
                        .mem
                        .set_u32(self.edi.wrapping_add(0x4), self.xmm[1][0].to_bits()); // MOVSS dword ptr [EDI + 0x4],XMM1
                    self.eax = (self.eax & 0xffffff00)
                        | u32::from(self.e.mem.u8(self.esi.wrapping_add(0x70))); // MOV AL,byte ptr [ESI + 0x70]
                    self.edi = self.add32(self.edi, 0xc); // ADD EDI,0xc
                    self.esi = self.add32(self.esi, 0x70); // ADD ESI,0x70
                    self.cmp8(self.eax as u8, 0x1); // CMP AL,0x1
                    if !self.zf {
                        // JNZ 0x00d8ab0c
                        block = 0x00d8_ab0c;
                        continue;
                    }
                    block = 0x00d8_bbaa;
                }
                0x00d8_bbaa => {
                    block = 0x00d8_aaf0; // JMP 0x00d8aaf0
                }
                0x00d8_bbaf => {
                    self.xmm[0] = load_vec(self.e, self.esi); // MOVAPS XMM0,xmmword ptr [ESI]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [EBX + 0x20]
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [ESI + 0x10]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ECX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[0] = add(self.xmm[0], self.xmm[2]); // ADDPS XMM0,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0x55); // SHUFPS XMM2,XMM0,0x55
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[0]); // ADDSS XMM2,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[2]); // ADDSS XMM0,XMM2
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0xff); // SHUFPS XMM1,XMM1,0xff
                    self.xmm[1] = sub_ss(self.xmm[1], self.xmm[0]); // SUBSS XMM1,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xbc), self.xmm[1][0].to_bits()); // MOVSS dword ptr [ESP + 0xbc],XMM1
                    self.fld32(self.esi.wrapping_add(0xc)); // FLD float ptr [ESI + 0xc]
                    self.fmul32(self.esp.wrapping_add(0xbc)); // FMUL float ptr [ESP + 0xbc]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.x87.fld_st(0); // FLD ST0
                    self.x87.fabs(); // FABS
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fld32(self.esi.wrapping_add(0x20)); // FLD float ptr [ESI + 0x20]
                    self.fcom_st(1, true); // FCOMP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d8bc38
                        block = 0x00d8_bc38;
                        continue;
                    }
                    block = 0x00d8_bc21;
                }
                0x00d8_bc21 => {
                    self.fdivr32(self.esi.wrapping_add(0x20)); // FDIVR float ptr [ESI + 0x20]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.x87.fld_st(0); // FLD ST0
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.fxch(1); // FXCH
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    block = 0x00d8_bc3e; // JMP 0x00d8bc3e
                }
                0x00d8_bc38 => {
                    self.x87.pop(); // FSTP ST0
                    self.x87.pop(); // FSTP ST0
                    self.x87.push(1.0); // FLD1
                    block = 0x00d8_bc3e;
                }
                0x00d8_bc3e => {
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.fstp32(self.edi.wrapping_add(0x4)); // FSTP float ptr [EDI + 0x4]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EBX + 0x30]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EAX + 0x30]
                    self.xmm[1] = self.xmm[2]; // MOVAPS XMM1,XMM2
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[2], 0x0); // SHUFPS XMM1,XMM2,0x0
                    self.xmm[3] = mul(self.xmm[3], self.xmm[1]); // MULPS XMM3,XMM1
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi); // MOVAPS XMM1,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM3,dword ptr [EDI]
                    self.xmm[4] = add(self.xmm[4], self.xmm[1]); // ADDPS XMM4,XMM1
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[4]); // MOVAPS xmmword ptr [EBX + 0x20],XMM4
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.eax.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EAX + 0x20],XMM0
                    self.xmm[3] = add_ss(self.xmm[3], self.xmm[2]); // ADDSS XMM3,XMM2
                    self.push(self.esi); // PUSH ESI
                    self.e.mem.set_u32(self.edi, self.xmm[3][0].to_bits()); // MOVSS dword ptr [EDI],XMM3
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.eax = (self.eax & 0xffffff00)
                        | u32::from(self.e.mem.u8(self.esi.wrapping_add(0x30))); // MOV AL,byte ptr [ESI + 0x30]
                    self.esp = self.add32(self.esp, 0x4); // ADD ESP,0x4
                    self.esi = self.add32(self.esi, 0x30); // ADD ESI,0x30
                    self.cmp8(self.eax as u8, 0xe); // CMP AL,0xe
                    if self.zf {
                        // JZ 0x00d8bbaf
                        block = 0x00d8_bbaf;
                        continue;
                    }
                    block = 0x00d8_bcac;
                }
                0x00d8_bcac => {
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_bcb1 => {
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x50)); // MOVAPS XMM0,xmmword ptr [EBX + 0x50]
                    self.fld32(self.esi.wrapping_add(0x28)); // FLD float ptr [ESI + 0x28]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EBX + 0x20]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.fld32(self.esi.wrapping_add(0x1c)); // FLD float ptr [ESI + 0x1c]
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x50)); // MOVAPS XMM2,xmmword ptr [ECX + 0x50]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x50)); // MOV EDX,dword ptr [EBX + 0x50]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.eax = self.e.mem.u32(self.ebp.wrapping_add(0x8)); // MOV EAX,dword ptr [EBP + 0x8]
                    self.x87.fld_st(0); // FLD ST0
                    self.xmm[3] = movss_load(self.e.mem.u32(self.eax.wrapping_add(0x8))); // MOVSS XMM3,dword ptr [EAX + 0x8]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ECX + 0x20]
                    self.xmm[0] = sub(self.xmm[0], self.xmm[2]); // SUBPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi); // MOVAPS XMM2,xmmword ptr [ESI]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[1]); // MULPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [ESI + 0x10]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x180), self.edx); // MOV dword ptr [ESP + 0x180],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x54)); // MOV EDX,dword ptr [EBX + 0x54]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x184), self.edx); // MOV dword ptr [ESP + 0x184],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x58)); // MOV EDX,dword ptr [EBX + 0x58]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x188), self.edx); // MOV dword ptr [ESP + 0x188],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x5c)); // MOV EDX,dword ptr [EBX + 0x5c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x18c), self.edx); // MOV dword ptr [ESP + 0x18c],EDX
                    self.edx = self.e.mem.u32(self.ecx.wrapping_add(0x50)); // MOV EDX,dword ptr [ECX + 0x50]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x100), self.edx); // MOV dword ptr [ESP + 0x100],EDX
                    self.edx = self.e.mem.u32(self.ecx.wrapping_add(0x54)); // MOV EDX,dword ptr [ECX + 0x54]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.e.mem.set_u32(self.esp.wrapping_add(0x104), self.edx); // MOV dword ptr [ESP + 0x104],EDX
                    self.edx = self.e.mem.u32(self.ecx.wrapping_add(0x58)); // MOV EDX,dword ptr [ECX + 0x58]
                    self.xmm[0] = add(self.xmm[0], self.xmm[2]); // ADDPS XMM0,XMM2
                    self.e.mem.set_u32(self.esp.wrapping_add(0x108), self.edx); // MOV dword ptr [ESP + 0x108],EDX
                    self.edx = self.e.mem.u32(self.ecx.wrapping_add(0x5c)); // MOV EDX,dword ptr [ECX + 0x5c]
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0x180)); // MOVAPS XMM0,xmmword ptr [ESP + 0x180]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x10c), self.edx); // MOV dword ptr [ESP + 0x10c],EDX
                    self.fmul32(self.eax.wrapping_add(0x40)); // FMUL float ptr [EAX + 0x40]
                    self.xmm[1] = load_vec(self.e, self.esi); // MOVAPS XMM1,xmmword ptr [ESI]
                    self.xmm[4] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM4,xmmword ptr [ESI + 0x10]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0x100)); // MOVAPS XMM0,xmmword ptr [ESP + 0x100]
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[4]); // MULPS XMM0,XMM4
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x2c))); // MOVSS XMM1,dword ptr [ESP + 0x2c]
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0x0); // SHUFPS XMM1,XMM1,0x0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = self.xmm[3]; // MOVAPS XMM0,XMM3
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0x0); // SHUFPS XMM0,XMM0,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xaa); // SHUFPS XMM2,XMM0,0xaa
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[1]); // ADDSS XMM2,XMM1
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x80), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0x80],XMM2
                    self.fsub32(self.esp.wrapping_add(0x80)); // FSUB float ptr [ESP + 0x80]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x2c))); // MOVSS XMM0,dword ptr [ESP + 0x2c]
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x88), self.xmm[0][0].to_bits()); // MOVSS dword ptr [ESP + 0x88],XMM0
                    self.fld32(self.edi); // FLD float ptr [EDI]
                    self.x87.fchs(); // FCHS
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x88)); // FLD float ptr [ESP + 0x88]
                    self.x87.fld_st(0); // FLD ST0
                    self.fsub32(self.esi.wrapping_add(0x20)); // FSUB float ptr [ESI + 0x20]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esi.wrapping_add(0xc)); // FLD float ptr [ESI + 0xc]
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fcom_st(1, false); // FCOM
                    self.fnstsw_ax(); // FNSTSW AX
                    self.x87.fstp_st(1); // FSTP ST1
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d8be3e
                        block = 0x00d8_be3e;
                        continue;
                    }
                    block = 0x00d8_bdfe;
                }
                0x00d8_bdfe => {
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.x87.fstp_st(1); // FSTP ST1
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EBX + 0x30]
                    self.x87.pop(); // FSTP ST0
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [ECX + 0x30]
                    self.xmm[1] = self.xmm[2]; // MOVAPS XMM1,XMM2
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[2], 0x0); // SHUFPS XMM1,XMM2,0x0
                    self.xmm[3] = mul(self.xmm[3], self.xmm[1]); // MULPS XMM3,XMM1
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi); // MOVAPS XMM1,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM3,dword ptr [EDI]
                    self.xmm[4] = add(self.xmm[4], self.xmm[1]); // ADDPS XMM4,XMM1
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[4]); // MOVAPS xmmword ptr [EBX + 0x20],XMM4
                    block = 0x00d8_be9b; // JMP 0x00d8be9b
                }
                0x00d8_be3e => {
                    self.fld32(self.esi.wrapping_add(0x24)); // FLD float ptr [ESI + 0x24]
                    self.x87.fsubp(2); // FSUBP ST2,ST0
                    self.x87.fxch(1); // FXCH
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esi.wrapping_add(0xc)); // FLD float ptr [ESI + 0xc]
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fcompp(); // FCOMPP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d8beae
                        block = 0x00d8_beae;
                        continue;
                    }
                    block = 0x00d8_be61;
                }
                0x00d8_be61 => {
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EBX + 0x30]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [ECX + 0x30]
                    self.xmm[1] = self.xmm[2]; // MOVAPS XMM1,XMM2
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[2], 0x0); // SHUFPS XMM1,XMM2,0x0
                    self.xmm[3] = mul(self.xmm[3], self.xmm[1]); // MULPS XMM3,XMM1
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi); // MOVAPS XMM1,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM3,dword ptr [EDI]
                    self.xmm[1] = add(self.xmm[1], self.xmm[4]); // ADDPS XMM1,XMM4
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [EBX + 0x20],XMM1
                    block = 0x00d8_be9b;
                }
                0x00d8_be9b => {
                    self.xmm[1] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [ECX + 0x20]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.ecx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [ECX + 0x20],XMM0
                    self.xmm[3] = add_ss(self.xmm[3], self.xmm[2]); // ADDSS XMM3,XMM2
                    self.e.mem.set_u32(self.edi, self.xmm[3][0].to_bits()); // MOVSS dword ptr [EDI],XMM3
                    block = 0x00d8_beae;
                }
                0x00d8_beae => {
                    self.push(self.esi); // PUSH ESI
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.eax = (self.eax & 0xffffff00)
                        | u32::from(self.e.mem.u8(self.esi.wrapping_add(0x30))); // MOV AL,byte ptr [ESI + 0x30]
                    self.esp = self.add32(self.esp, 0x4); // ADD ESP,0x4
                    self.esi = self.add32(self.esi, 0x30); // ADD ESI,0x30
                    self.cmp8(self.eax as u8, 0xd); // CMP AL,0xd
                    if self.zf {
                        // JZ 0x00d8bcb1
                        block = 0x00d8_bcb1;
                        continue;
                    }
                    block = 0x00d8_bec8;
                }
                0x00d8_bec8 => {
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_becd => {
                    self.eax = self.esi.wrapping_add(0x24); // LEA EAX,[ESI + 0x24]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.eax); // MOV dword ptr [ESP + 0x30],EAX
                    block = 0x00d8_bed4;
                }
                0x00d8_bed4 => {
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV ECX,dword ptr [ESP + 0x30]
                    self.fld32(self.ecx.wrapping_add(0x4)); // FLD float ptr [ECX + 0x4]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.fmul32(self.esp.wrapping_add(0x78)); // FMUL float ptr [ESP + 0x78]
                    self.fadd32(self.edi.wrapping_add(0x8)); // FADD float ptr [EDI + 0x8]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fst32(self.esp.wrapping_add(0x10)); // FST float ptr [ESP + 0x10]
                    self.fstp32(self.edi.wrapping_add(0x8)); // FSTP float ptr [EDI + 0x8]
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x50)); // MOVAPS XMM0,xmmword ptr [EBX + 0x50]
                    self.fld32(self.ecx.wrapping_sub(0x8)); // FLD float ptr [ECX + -0x8]
                    self.xmm[3] = load_vec(self.e, self.eax.wrapping_add(0x50)); // MOVAPS XMM3,xmmword ptr [EAX + 0x50]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EBX + 0x20]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[2] = movss_load(self.e.mem.u32(self.ecx.wrapping_add(0xc))); // MOVSS XMM2,dword ptr [ECX + 0xc]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fadd32(self.esp.wrapping_add(0x10)); // FADD float ptr [ESP + 0x10]
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x50)); // MOV EDX,dword ptr [EBX + 0x50]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EAX + 0x20]
                    self.xmm[0] = sub(self.xmm[0], self.xmm[3]); // SUBPS XMM0,XMM3
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esi); // MOVAPS XMM3,xmmword ptr [ESI]
                    self.fld32(self.ecx.wrapping_add(0x8)); // FLD float ptr [ECX + 0x8]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[3] = load_vec(self.e, self.ecx.wrapping_sub(0x14)); // MOVAPS XMM3,xmmword ptr [ECX + -0x14]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xe0), self.edx); // MOV dword ptr [ESP + 0xe0],EDX
                    self.x87.fld_st(0); // FLD ST0
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x54)); // MOV EDX,dword ptr [EBX + 0x54]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xe4), self.edx); // MOV dword ptr [ESP + 0xe4],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x58)); // MOV EDX,dword ptr [EBX + 0x58]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xe8), self.edx); // MOV dword ptr [ESP + 0xe8],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x5c)); // MOV EDX,dword ptr [EBX + 0x5c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xec), self.edx); // MOV dword ptr [ESP + 0xec],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x50)); // MOV EDX,dword ptr [EAX + 0x50]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x120), self.edx); // MOV dword ptr [ESP + 0x120],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x54)); // MOV EDX,dword ptr [EAX + 0x54]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x124), self.edx); // MOV dword ptr [ESP + 0x124],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x58)); // MOV EDX,dword ptr [EAX + 0x58]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x128), self.edx); // MOV dword ptr [ESP + 0x128],EDX
                    self.eax = self.e.mem.u32(self.eax.wrapping_add(0x5c)); // MOV EAX,dword ptr [EAX + 0x5c]
                    self.edx = self.e.mem.u32(self.ebp.wrapping_add(0x8)); // MOV EDX,dword ptr [EBP + 0x8]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esp.wrapping_add(0xe0)); // MOVAPS XMM1,xmmword ptr [ESP + 0xe0]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x12c), self.eax); // MOV dword ptr [ESP + 0x12c],EAX
                    self.fmul32(self.edx.wrapping_add(0x40)); // FMUL float ptr [EDX + 0x40]
                    self.xmm[4] = load_vec(self.e, self.ecx.wrapping_sub(0x14)); // MOVAPS XMM4,xmmword ptr [ECX + -0x14]
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi); // MOVAPS XMM0,xmmword ptr [ESI]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0x120)); // MOVAPS XMM0,xmmword ptr [ESP + 0x120]
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[4]); // MULPS XMM0,XMM4
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x2c))); // MOVSS XMM1,dword ptr [ESP + 0x2c]
                    self.eax = self.ecx.wrapping_sub(0x14); // LEA EAX,[ECX + -0x14]
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0x0); // SHUFPS XMM1,XMM1,0x0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0][0] = self.xmm[2][0]; // MOVSS XMM0,XMM2
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0x0); // SHUFPS XMM0,XMM0,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xaa); // SHUFPS XMM2,XMM0,0xaa
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[1]); // ADDSS XMM2,XMM1
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x90), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0x90],XMM2
                    self.fsub32(self.esp.wrapping_add(0x90)); // FSUB float ptr [ESP + 0x90]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x50), self.eax); // MOV dword ptr [ESP + 0x50],EAX
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x2c))); // MOVSS XMM0,dword ptr [ESP + 0x2c]
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x98), self.xmm[0][0].to_bits()); // MOVSS dword ptr [ESP + 0x98],XMM0
                    self.fld32(self.ecx.wrapping_sub(0x18)); // FLD float ptr [ECX + -0x18]
                    self.fmul32(self.esp.wrapping_add(0x98)); // FMUL float ptr [ESP + 0x98]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.edx = self.ecx.wrapping_sub(0x4); // LEA EDX,[ECX + -0x4]
                    self.fld32(self.edx); // FLD float ptr [EDX]
                    self.fcom_st(1, true); // FCOMP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d8c049
                        block = 0x00d8_c049;
                        continue;
                    }
                    block = 0x00d8_c03c;
                }
                0x00d8_c03c => {
                    self.fdivr32(self.edx); // FDIVR float ptr [EDX]
                    self.fstp32(self.edi.wrapping_add(0x4)); // FSTP float ptr [EDI + 0x4]
                    self.fld32(self.edx); // FLD float ptr [EDX]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    block = 0x00d8_c068; // JMP 0x00d8c068
                }
                0x00d8_c049 => {
                    self.fld32(self.ecx); // FLD float ptr [ECX]
                    self.fcom_st(1, true); // FCOMP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x41); // TEST AH,0x41
                    if !self.zf {
                        // JNZ 0x00d8c061
                        block = 0x00d8_c061;
                        continue;
                    }
                    block = 0x00d8_c054;
                }
                0x00d8_c054 => {
                    self.fdivr32(self.ecx); // FDIVR float ptr [ECX]
                    self.fstp32(self.edi.wrapping_add(0x4)); // FSTP float ptr [EDI + 0x4]
                    self.fld32(self.ecx); // FLD float ptr [ECX]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    block = 0x00d8_c068; // JMP 0x00d8c068
                }
                0x00d8_c061 => {
                    self.x87.pop(); // FSTP ST0
                    self.x87.push(1.0); // FLD1
                    self.fstp32(self.edi.wrapping_add(0x4)); // FSTP float ptr [EDI + 0x4]
                    block = 0x00d8_c068;
                }
                0x00d8_c068 => {
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EBX + 0x30]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x50)); // MOV ECX,dword ptr [ESP + 0x50]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[2], 0x0); // SHUFPS XMM0,XMM2,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EAX + 0x30]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi); // MOVAPS XMM3,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = load_vec(self.e, self.ecx); // MOVAPS XMM3,xmmword ptr [ECX]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM3,dword ptr [EDI]
                    self.xmm[1] = add(self.xmm[1], self.xmm[4]); // ADDPS XMM1,XMM4
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [EBX + 0x20],XMM1
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.eax.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EAX + 0x20],XMM0
                    self.xmm[3] = add_ss(self.xmm[3], self.xmm[2]); // ADDSS XMM3,XMM2
                    self.push(self.esi); // PUSH ESI
                    self.e.mem.set_u32(self.edi, self.xmm[3][0].to_bits()); // MOVSS dword ptr [EDI],XMM3
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x34)); // MOV ECX,dword ptr [ESP + 0x34]
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.eax = (self.eax & 0xffffff00)
                        | u32::from(self.e.mem.u8(self.ecx.wrapping_add(0x1c))); // MOV AL,byte ptr [ECX + 0x1c]
                    self.ecx = self.add32(self.ecx, 0x40); // ADD ECX,0x40
                    self.esp = self.add32(self.esp, 0x4); // ADD ESP,0x4
                    self.esi = self.add32(self.esi, 0x40); // ADD ESI,0x40
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.ecx); // MOV dword ptr [ESP + 0x30],ECX
                    self.cmp8(self.eax as u8, 0xf); // CMP AL,0xf
                    if self.zf {
                        // JZ 0x00d8bed4
                        block = 0x00d8_bed4;
                        continue;
                    }
                    block = 0x00d8_c0e1;
                }
                0x00d8_c0e1 => {
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_c0f0 => {
                    self.xmm[2] = load_vec(self.e, self.esi); // MOVAPS XMM2,xmmword ptr [ESI]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EDX,dword ptr [ESP + 0x4c]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = load_vec(self.e, self.edx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EDX + 0x20]
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM5,xmmword ptr [EBX + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[4] = shufps(self.xmm[4], self.xmm[0], 0x55); // SHUFPS XMM4,XMM0,0x55
                    self.eax = self.ebx.wrapping_add(0x20); // LEA EAX,[EBX + 0x20]
                    self.ecx = self.edx.wrapping_add(0x20); // LEA ECX,[EDX + 0x20]
                    self.xmm[4] = add_ss(self.xmm[4], self.xmm[0]); // ADDSS XMM4,XMM0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0xaa); // SHUFPS XMM1,XMM0,0xaa
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[4]); // ADDSS XMM1,XMM4
                    self.xmm[0] = self.xmm[3]; // MOVAPS XMM0,XMM3
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[3], 0xff); // SHUFPS XMM0,XMM3,0xff
                    self.xmm[0] = sub_ss(self.xmm[0], self.xmm[1]); // SUBSS XMM0,XMM1
                    self.xmm[1] = self.xmm[2]; // MOVAPS XMM1,XMM2
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[2], 0xff); // SHUFPS XMM1,XMM2,0xff
                    self.xmm[0] = mul_ss(self.xmm[0], self.xmm[1]); // MULSS XMM0,XMM1
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[4], 0x0); // SHUFPS XMM0,XMM4,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[5]); // MULPS XMM1,XMM5
                    self.xmm[5] = load_vec(self.e, self.edx.wrapping_add(0x30)); // MOVAPS XMM5,xmmword ptr [EDX + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[5]); // MULPS XMM0,XMM5
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.eax); // MOVAPS XMM3,xmmword ptr [EAX]
                    self.xmm[1] = add(self.xmm[1], self.xmm[3]); // ADDPS XMM1,XMM3
                    store_vec(self.e, self.eax, self.xmm[1]); // MOVAPS xmmword ptr [EAX],XMM1
                    self.xmm[1] = load_vec(self.e, self.ecx); // MOVAPS XMM1,xmmword ptr [ECX]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.ecx, self.xmm[0]); // MOVAPS xmmword ptr [ECX],XMM0
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[4]); // ADDSS XMM2,XMM4
                    self.push(self.esi); // PUSH ESI
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.eax = (self.eax & 0xffffff00)
                        | u32::from(self.e.mem.u8(self.esi.wrapping_add(0x20))); // MOV AL,byte ptr [ESI + 0x20]
                    self.esp = self.add32(self.esp, 0x4); // ADD ESP,0x4
                    self.esi = self.add32(self.esi, 0x20); // ADD ESI,0x20
                    self.cmp8(self.eax as u8, 0xc); // CMP AL,0xc
                    if self.zf {
                        // JZ 0x00d8c0f0
                        block = 0x00d8_c0f0;
                        continue;
                    }
                    block = 0x00d8_c194;
                }
                0x00d8_c194 => {
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_c1a0 => {
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EBX + 0x10]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [ESI + 0x20]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EDX,dword ptr [ESP + 0x4c]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[0]); // SUBPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.edx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EDX + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi); // MOVAPS XMM0,xmmword ptr [ESI]
                    self.xmm[4] = mul(self.xmm[4], self.xmm[3]); // MULPS XMM4,XMM3
                    self.xmm[1] = add(self.xmm[1], self.xmm[4]); // ADDPS XMM1,XMM4
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[1] = add(self.xmm[1], self.xmm[2]); // ADDPS XMM1,XMM2
                    self.eax = self.ebx.wrapping_add(0x10); // LEA EAX,[EBX + 0x10]
                    self.ecx = self.add32(self.ecx, 0x10); // ADD ECX,0x10
                    self.xmm[4] = self.xmm[1]; // MOVAPS XMM4,XMM1
                    self.xmm[4] = shufps(self.xmm[4], self.xmm[1], 0x55); // SHUFPS XMM4,XMM1,0x55
                    self.edx = self.add32(self.edx, 0x20); // ADD EDX,0x20
                    self.xmm[4] = add_ss(self.xmm[4], self.xmm[1]); // ADDSS XMM4,XMM1
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[1], 0xaa); // SHUFPS XMM2,XMM1,0xaa
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[3], 0xff); // SHUFPS XMM3,XMM3,0xff
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[4]); // ADDSS XMM2,XMM4
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0xff); // SHUFPS XMM1,XMM0,0xff
                    self.xmm[1] = sub_ss(self.xmm[1], self.xmm[2]); // SUBSS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.xmm[1] = mul_ss(self.xmm[1], self.xmm[3]); // MULSS XMM1,XMM3
                    self.xmm[4] = self.xmm[1]; // MOVAPS XMM4,XMM1
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.edx); // MOV dword ptr [ESP + 0x30],EDX
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EDX,dword ptr [ESP + 0x4c]
                    self.xmm[3] = self.xmm[4]; // MOVAPS XMM3,XMM4
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[4], 0x0); // SHUFPS XMM3,XMM4,0x0
                    self.xmm[1] = self.xmm[3]; // MOVAPS XMM1,XMM3
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.edx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EDX + 0x30]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[3]); // MULPS XMM2,XMM3
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[6] = self.xmm[0]; // MOVAPS XMM6,XMM0
                    self.xmm[6] = mul(self.xmm[6], self.xmm[3]); // MULPS XMM6,XMM3
                    self.xmm[3] = load_vec(self.e, self.eax); // MOVAPS XMM3,xmmword ptr [EAX]
                    self.xmm[3] = add(self.xmm[3], self.xmm[6]); // ADDPS XMM3,XMM6
                    store_vec(self.e, self.eax, self.xmm[3]); // MOVAPS xmmword ptr [EAX],XMM3
                    self.xmm[3] = load_vec(self.e, self.ecx); // MOVAPS XMM3,xmmword ptr [ECX]
                    self.xmm[5] = self.xmm[2]; // MOVAPS XMM5,XMM2
                    self.xmm[5] = shufps(self.xmm[5], self.xmm[2], 0xff); // SHUFPS XMM5,XMM2,0xff
                    self.xmm[0] = mul(self.xmm[0], self.xmm[5]); // MULPS XMM0,XMM5
                    self.xmm[3] = sub(self.xmm[3], self.xmm[0]); // SUBPS XMM3,XMM0
                    store_vec(self.e, self.ecx, self.xmm[3]); // MOVAPS xmmword ptr [ECX],XMM3
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.eax = self.ebx.wrapping_add(0x20); // LEA EAX,[EBX + 0x20]
                    self.xmm[1] = add(self.xmm[1], self.xmm[3]); // ADDPS XMM1,XMM3
                    store_vec(self.e, self.eax, self.xmm[1]); // MOVAPS xmmword ptr [EAX],XMM1
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EAX,dword ptr [ESP + 0x30]
                    self.xmm[1] = load_vec(self.e, self.eax); // MOVAPS XMM1,xmmword ptr [EAX]
                    self.xmm[1] = add(self.xmm[1], self.xmm[0]); // ADDPS XMM1,XMM0
                    store_vec(self.e, self.eax, self.xmm[1]); // MOVAPS xmmword ptr [EAX],XMM1
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[4]); // ADDSS XMM2,XMM4
                    self.push(self.esi); // PUSH ESI
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.esi = self.add32(self.esi, 0x30); // ADD ESI,0x30
                    self.eax = self.esp.wrapping_add(0x76); // LEA EAX,[ESP + 0x76]
                    self.push(self.esi); // PUSH ESI
                    self.push(self.eax); // PUSH EAX
                    self.call_copy_byte(); // CALL 0x00d921c0
                    self.eax = i32::from(self.e.mem.i8(self.eax)) as u32; // MOVSX EAX,byte ptr [EAX]
                    self.esp = self.add32(self.esp, 0xc); // ADD ESP,0xc
                    self.cmp32(self.eax, 0x5); // CMP EAX,0x5
                    if self.zf {
                        // JZ 0x00d8c1a0
                        block = 0x00d8_c1a0;
                        continue;
                    }
                    block = 0x00d8_c2ad;
                }
                0x00d8_c2ad => {
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_c2b2 => {
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EDX,dword ptr [ESP + 0x4c]
                    self.xmm[3] = load_vec(self.e, self.edx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EDX + 0x20]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi); // MOVAPS XMM3,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[4] = mul(self.xmm[4], self.xmm[2]); // MULPS XMM4,XMM2
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.xmm[0] = add(self.xmm[0], self.xmm[4]); // ADDPS XMM0,XMM4
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.eax = self.ebx.wrapping_add(0x10); // LEA EAX,[EBX + 0x10]
                    self.ecx = self.add32(self.ecx, 0x10); // ADD ECX,0x10
                    self.edx = self.add32(self.edx, 0x20); // ADD EDX,0x20
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[3], 0xff); // SHUFPS XMM3,XMM3,0xff
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[3] = sub_ss(self.xmm[3], self.xmm[0]); // SUBSS XMM3,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM0,dword ptr [EDI]
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[3] = mul_ss(self.xmm[3], self.xmm[2]); // MULSS XMM3,XMM2
                    self.xmm[2] = load_vec(self.e, VECTOR_CONSTANT_010C7160); // MOVAPS XMM2,xmmword ptr [0x010c7160]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.edx); // MOV dword ptr [ESP + 0x30],EDX
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EDX,dword ptr [ESP + 0x4c]
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xff); // SHUFPS XMM0,XMM0,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[1]); // SUBSS XMM2,XMM1
                    self.xmm[2] = max_ps(self.xmm[2], self.xmm[3]); // MAXPS XMM2,XMM3
                    self.xmm[0] = sub_ss(self.xmm[0], self.xmm[1]); // SUBSS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [EBX + 0x30]
                    self.xmm[0] = min_ps(self.xmm[0], self.xmm[2]); // MINPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.edx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EDX + 0x30]
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[0], 0x0); // SHUFPS XMM3,XMM0,0x0
                    self.xmm[2] = mul(self.xmm[2], self.xmm[3]); // MULPS XMM2,XMM3
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[4] = self.xmm[2]; // MOVAPS XMM4,XMM2
                    self.xmm[4] = shufps(self.xmm[4], self.xmm[2], 0xff); // SHUFPS XMM4,XMM2,0xff
                    self.xmm[4] = mul(self.xmm[4], self.xmm[5]); // MULPS XMM4,XMM5
                    self.xmm[5] = load_vec(self.e, self.eax); // MOVAPS XMM5,xmmword ptr [EAX]
                    self.xmm[5] = add(self.xmm[5], self.xmm[3]); // ADDPS XMM5,XMM3
                    store_vec(self.e, self.eax, self.xmm[5]); // MOVAPS xmmword ptr [EAX],XMM5
                    self.xmm[3] = load_vec(self.e, self.ecx); // MOVAPS XMM3,xmmword ptr [ECX]
                    self.xmm[3] = sub(self.xmm[3], self.xmm[4]); // SUBPS XMM3,XMM4
                    store_vec(self.e, self.ecx, self.xmm[3]); // MOVAPS xmmword ptr [ECX],XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[1]); // MULPS XMM3,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [ESI + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.eax = self.ebx.wrapping_add(0x20); // LEA EAX,[EBX + 0x20]
                    self.xmm[3] = add(self.xmm[3], self.xmm[4]); // ADDPS XMM3,XMM4
                    store_vec(self.e, self.eax, self.xmm[3]); // MOVAPS xmmword ptr [EAX],XMM3
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EAX,dword ptr [ESP + 0x30]
                    self.xmm[3] = load_vec(self.e, self.eax); // MOVAPS XMM3,xmmword ptr [EAX]
                    self.xmm[3] = add(self.xmm[3], self.xmm[1]); // ADDPS XMM3,XMM1
                    store_vec(self.e, self.eax, self.xmm[3]); // MOVAPS xmmword ptr [EAX],XMM3
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[0]); // ADDSS XMM2,XMM0
                    self.push(self.esi); // PUSH ESI
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.esi = self.add32(self.esi, 0x30); // ADD ESI,0x30
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.eax = self.esp.wrapping_add(0x77); // LEA EAX,[ESP + 0x77]
                    self.push(self.esi); // PUSH ESI
                    self.push(self.eax); // PUSH EAX
                    self.call_copy_byte(); // CALL 0x00d921c0
                    self.eax = i32::from(self.e.mem.i8(self.eax)) as u32; // MOVSX EAX,byte ptr [EAX]
                    self.esp = self.add32(self.esp, 0xc); // ADD ESP,0xc
                    self.cmp32(self.eax, 0x6); // CMP EAX,0x6
                    if self.zf {
                        // JZ 0x00d8c2b2
                        block = 0x00d8_c2b2;
                        continue;
                    }
                    block = 0x00d8_c3db;
                }
                0x00d8_c3db => {
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_c3e0 => {
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x40)); // MOVAPS XMM1,xmmword ptr [EBX + 0x40]
                    self.fld32(self.esi.wrapping_add(0x30)); // FLD float ptr [ESI + 0x30]
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [EBX + 0x10]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x50)); // MOVAPS XMM2,xmmword ptr [EBX + 0x50]
                    self.fld32(self.esi.wrapping_add(0xc)); // FLD float ptr [ESI + 0xc]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.fstp32(self.esp.wrapping_add(0x50)); // FSTP float ptr [ESP + 0x50]
                    self.xmm[3] = load_vec(self.e, self.eax.wrapping_add(0x40)); // MOVAPS XMM3,xmmword ptr [EAX + 0x40]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.xmm[5] = load_vec(self.e, self.eax.wrapping_add(0x50)); // MOVAPS XMM5,xmmword ptr [EAX + 0x50]
                    self.x87.fld_st(0); // FLD ST0
                    self.xmm[4] = movss_load(self.e.mem.u32(self.esi.wrapping_add(0x34))); // MOVSS XMM4,dword ptr [ESI + 0x34]
                    self.xmm[0] = sub(self.xmm[0], self.xmm[1]); // SUBPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EBX + 0x20]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[2]); // SUBPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    self.xmm[3] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EAX + 0x20]
                    self.xmm[0] = sub(self.xmm[0], self.xmm[2]); // SUBPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi); // MOVAPS XMM2,xmmword ptr [ESI]
                    self.ecx = self.ebx.wrapping_add(0x10); // LEA ECX,[EBX + 0x10]
                    self.edx = self.eax.wrapping_add(0x10); // LEA EDX,[EAX + 0x10]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x10), self.edx); // MOV dword ptr [ESP + 0x10],EDX
                    self.edx = self.eax.wrapping_add(0x20); // LEA EDX,[EAX + 0x20]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.edx); // MOV dword ptr [ESP + 0x30],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x40)); // MOV EDX,dword ptr [EBX + 0x40]
                    self.xmm[3] = sub(self.xmm[3], self.xmm[5]); // SUBPS XMM3,XMM5
                    self.xmm[5] = self.xmm[0]; // MOVAPS XMM5,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x160), self.edx); // MOV dword ptr [ESP + 0x160],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x44)); // MOV EDX,dword ptr [EBX + 0x44]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x164), self.edx); // MOV dword ptr [ESP + 0x164],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x48)); // MOV EDX,dword ptr [EBX + 0x48]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x168), self.edx); // MOV dword ptr [ESP + 0x168],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x4c)); // MOV EDX,dword ptr [EBX + 0x4c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x16c), self.edx); // MOV dword ptr [ESP + 0x16c],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x50)); // MOV EDX,dword ptr [EBX + 0x50]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xf0), self.edx); // MOV dword ptr [ESP + 0xf0],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x54)); // MOV EDX,dword ptr [EBX + 0x54]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xf4), self.edx); // MOV dword ptr [ESP + 0xf4],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x58)); // MOV EDX,dword ptr [EBX + 0x58]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xf8), self.edx); // MOV dword ptr [ESP + 0xf8],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x5c)); // MOV EDX,dword ptr [EBX + 0x5c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xfc), self.edx); // MOV dword ptr [ESP + 0xfc],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x40)); // MOV EDX,dword ptr [EAX + 0x40]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x140), self.edx); // MOV dword ptr [ESP + 0x140],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x44)); // MOV EDX,dword ptr [EAX + 0x44]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x144), self.edx); // MOV dword ptr [ESP + 0x144],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x48)); // MOV EDX,dword ptr [EAX + 0x48]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x148), self.edx); // MOV dword ptr [ESP + 0x148],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x4c)); // MOV EDX,dword ptr [EAX + 0x4c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x14c), self.edx); // MOV dword ptr [ESP + 0x14c],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x50)); // MOV EDX,dword ptr [EAX + 0x50]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.e.mem.set_u32(self.esp.wrapping_add(0x110), self.edx); // MOV dword ptr [ESP + 0x110],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x54)); // MOV EDX,dword ptr [EAX + 0x54]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esp.wrapping_add(0x160)); // MOVAPS XMM1,xmmword ptr [ESP + 0x160]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x114), self.edx); // MOV dword ptr [ESP + 0x114],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x58)); // MOV EDX,dword ptr [EAX + 0x58]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[0] = add(self.xmm[0], self.xmm[2]); // ADDPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esp.wrapping_add(0xf0)); // MOVAPS XMM2,xmmword ptr [ESP + 0xf0]
                    self.xmm[5] = self.xmm[0]; // MOVAPS XMM5,XMM0
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0x140)); // MOVAPS XMM0,xmmword ptr [ESP + 0x140]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x118), self.edx); // MOV dword ptr [ESP + 0x118],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x5c)); // MOV EDX,dword ptr [EAX + 0x5c]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.e.mem.set_u32(self.esp.wrapping_add(0x11c), self.edx); // MOV dword ptr [ESP + 0x11c],EDX
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esp.wrapping_add(0x110)); // MOVAPS XMM3,xmmword ptr [ESP + 0x110]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi); // MOVAPS XMM3,xmmword ptr [ESI]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[1]); // MULPS XMM3,XMM1
                    self.xmm[0] = add(self.xmm[0], self.xmm[2]); // ADDPS XMM0,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.edx = self.e.mem.u32(self.ebp.wrapping_add(0x8)); // MOV EDX,dword ptr [EBP + 0x8]
                    self.fmul32(self.edx.wrapping_add(0x40)); // FMUL float ptr [EDX + 0x40]
                    self.push(self.esi); // PUSH ESI
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.xmm[1] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x30))); // MOVSS XMM1,dword ptr [ESP + 0x30]
                    self.fmul32(self.esp.wrapping_add(0x54)); // FMUL float ptr [ESP + 0x54]
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0x0); // SHUFPS XMM1,XMM1,0x0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0][0] = self.xmm[4][0]; // MOVSS XMM0,XMM4
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0x0); // SHUFPS XMM0,XMM0,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[5]); // MULPS XMM0,XMM5
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xaa); // SHUFPS XMM2,XMM0,0xaa
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[1]); // ADDSS XMM2,XMM1
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xa4), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0xa4],XMM2
                    self.fsub32(self.esp.wrapping_add(0xa4)); // FSUB float ptr [ESP + 0xa4]
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xff); // SHUFPS XMM0,XMM0,0xff
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.xmm[1] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x30))); // MOVSS XMM1,dword ptr [ESP + 0x30]
                    self.xmm[1] = mul_ss(self.xmm[1], self.xmm[0]); // MULSS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[4] = self.xmm[1]; // MOVAPS XMM4,XMM1
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [EAX + 0x30]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x14)); // MOV EAX,dword ptr [ESP + 0x14]
                    self.xmm[2] = self.xmm[4]; // MOVAPS XMM2,XMM4
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[4], 0x0); // SHUFPS XMM2,XMM4,0x0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ecx); // MOVAPS XMM5,xmmword ptr [ECX]
                    self.xmm[5] = add(self.xmm[5], self.xmm[2]); // ADDPS XMM5,XMM2
                    store_vec(self.e, self.ecx, self.xmm[5]); // MOVAPS xmmword ptr [ECX],XMM5
                    self.xmm[2] = load_vec(self.e, self.eax); // MOVAPS XMM2,xmmword ptr [EAX]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.eax, self.xmm[2]); // MOVAPS xmmword ptr [EAX],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ESI + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[1]); // MULPS XMM2,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM1,dword ptr [EDI]
                    self.eax = self.ebx.wrapping_add(0x20); // LEA EAX,[EBX + 0x20]
                    self.xmm[3] = add(self.xmm[3], self.xmm[0]); // ADDPS XMM3,XMM0
                    store_vec(self.e, self.eax, self.xmm[3]); // MOVAPS xmmword ptr [EAX],XMM3
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x34)); // MOV EAX,dword ptr [ESP + 0x34]
                    self.xmm[0] = load_vec(self.e, self.eax); // MOVAPS XMM0,xmmword ptr [EAX]
                    self.xmm[0] = add(self.xmm[0], self.xmm[2]); // ADDPS XMM0,XMM2
                    store_vec(self.e, self.eax, self.xmm[0]); // MOVAPS xmmword ptr [EAX],XMM0
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[4]); // ADDSS XMM1,XMM4
                    self.e.mem.set_u32(self.edi, self.xmm[1][0].to_bits()); // MOVSS dword ptr [EDI],XMM1
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.esi = self.add32(self.esi, 0x40); // ADD ESI,0x40
                    self.eax = self.esp.wrapping_add(0x75); // LEA EAX,[ESP + 0x75]
                    self.push(self.esi); // PUSH ESI
                    self.push(self.eax); // PUSH EAX
                    self.call_copy_byte(); // CALL 0x00d921c0
                    self.eax = i32::from(self.e.mem.i8(self.eax)) as u32; // MOVSX EAX,byte ptr [EAX]
                    self.esp = self.add32(self.esp, 0xc); // ADD ESP,0xc
                    self.cmp32(self.eax, 0x7); // CMP EAX,0x7
                    if self.zf {
                        // JZ 0x00d8c3e0
                        block = 0x00d8_c3e0;
                        continue;
                    }
                    block = 0x00d8_c63e;
                }
                0x00d8_c63e => {
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_c643 => {
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [EAX + 0x10]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [EAX + 0x20]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi); // MOVAPS XMM2,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xa8), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0xa8],XMM2
                    self.fld32(self.esi.wrapping_add(0x1c)); // FLD float ptr [ESI + 0x1c]
                    self.fmul32(self.esp.wrapping_add(0xa8)); // FMUL float ptr [ESP + 0xa8]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.x87.fld_st(0); // FLD ST0
                    self.x87.fabs(); // FABS
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fld32(self.esi.wrapping_add(0x30)); // FLD float ptr [ESI + 0x30]
                    self.fcom_st(1, true); // FCOMP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d8c6e1
                        block = 0x00d8_c6e1;
                        continue;
                    }
                    block = 0x00d8_c6ca;
                }
                0x00d8_c6ca => {
                    self.fdivr32(self.esi.wrapping_add(0x30)); // FDIVR float ptr [ESI + 0x30]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.x87.fld_st(0); // FLD ST0
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.fxch(1); // FXCH
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    block = 0x00d8_c6e7; // JMP 0x00d8c6e7
                }
                0x00d8_c6e1 => {
                    self.x87.pop(); // FSTP ST0
                    self.x87.pop(); // FSTP ST0
                    self.x87.push(1.0); // FLD1
                    block = 0x00d8_c6e7;
                }
                0x00d8_c6e7 => {
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.fstp32(self.edi.wrapping_add(0x4)); // FSTP float ptr [EDI + 0x4]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [EAX + 0x30]
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[2] = self.xmm[4]; // MOVAPS XMM2,XMM4
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[4], 0x0); // SHUFPS XMM2,XMM4,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[5] = add(self.xmm[5], self.xmm[2]); // ADDPS XMM5,XMM2
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[5]); // MOVAPS xmmword ptr [EBX + 0x10],XMM5
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.eax.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EAX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ESI + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x20],XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EAX + 0x20]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.eax.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EAX + 0x20],XMM0
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[4]); // ADDSS XMM2,XMM4
                    self.push(self.esi); // PUSH ESI
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.esi = self.add32(self.esi, 0x40); // ADD ESI,0x40
                    self.ecx = self.esp.wrapping_add(0x73); // LEA ECX,[ESP + 0x73]
                    self.push(self.esi); // PUSH ESI
                    self.push(self.ecx); // PUSH ECX
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.call_copy_byte(); // CALL 0x00d921c0
                    self.eax = i32::from(self.e.mem.i8(self.eax)) as u32; // MOVSX EAX,byte ptr [EAX]
                    self.esp = self.add32(self.esp, 0xc); // ADD ESP,0xc
                    self.cmp32(self.eax, 0x9); // CMP EAX,0x9
                    if self.zf {
                        // JZ 0x00d8c643
                        block = 0x00d8_c643;
                        continue;
                    }
                    block = 0x00d8_c78f;
                }
                0x00d8_c78f => {
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_c794 => {
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ECX + 0x20]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi); // MOVAPS XMM2,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xb0), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0xb0],XMM2
                    self.fld32(self.esp.wrapping_add(0xb0)); // FLD float ptr [ESP + 0xb0]
                    self.fst32(self.esp.wrapping_add(0x30)); // FST float ptr [ESP + 0x30]
                    self.fld32(self.edi); // FLD float ptr [EDI]
                    self.x87.fchs(); // FCHS
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fsub32(self.esi.wrapping_add(0x30)); // FSUB float ptr [ESI + 0x30]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fmul32(self.esi.wrapping_add(0x1c)); // FMUL float ptr [ESI + 0x1c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fcom_st(1, false); // FCOM
                    self.fnstsw_ax(); // FNSTSW AX
                    self.x87.fstp_st(1); // FSTP ST1
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d8c8a0
                        block = 0x00d8_c8a0;
                        continue;
                    }
                    block = 0x00d8_c825;
                }
                0x00d8_c825 => {
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[1] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [ECX + 0x30]
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[2] = self.xmm[4]; // MOVAPS XMM2,XMM4
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[4], 0x0); // SHUFPS XMM2,XMM4,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[5]); // ADDPS XMM2,XMM5
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ECX + 0x10]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [ECX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ESI + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x20],XMM0
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ECX + 0x20]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.ecx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [ECX + 0x20],XMM0
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[4]); // ADDSS XMM2,XMM4
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    block = 0x00d8_c8a0;
                }
                0x00d8_c8a0 => {
                    self.fld32(self.esp.wrapping_add(0x30)); // FLD float ptr [ESP + 0x30]
                    self.fsub32(self.esi.wrapping_add(0x34)); // FSUB float ptr [ESI + 0x34]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fmul32(self.esi.wrapping_add(0x1c)); // FMUL float ptr [ESI + 0x1c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fcompp(); // FCOMPP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d8c942
                        block = 0x00d8_c942;
                        continue;
                    }
                    block = 0x00d8_c8c7;
                }
                0x00d8_c8c7 => {
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[1] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [ECX + 0x30]
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[2] = self.xmm[4]; // MOVAPS XMM2,XMM4
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[4], 0x0); // SHUFPS XMM2,XMM4,0x0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[5] = add(self.xmm[5], self.xmm[2]); // ADDPS XMM5,XMM2
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[5]); // MOVAPS xmmword ptr [EBX + 0x10],XMM5
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ECX + 0x10]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [ECX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ESI + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x20],XMM0
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ECX + 0x20]
                    self.xmm[1] = add(self.xmm[1], self.xmm[0]); // ADDPS XMM1,XMM0
                    store_vec(self.e, self.ecx.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [ECX + 0x20],XMM1
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[4]); // ADDSS XMM2,XMM4
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    block = 0x00d8_c942;
                }
                0x00d8_c942 => {
                    self.push(self.esi); // PUSH ESI
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.esi = self.add32(self.esi, 0x40); // ADD ESI,0x40
                    self.edx = self.esp.wrapping_add(0x74); // LEA EDX,[ESP + 0x74]
                    self.push(self.esi); // PUSH ESI
                    self.push(self.edx); // PUSH EDX
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.call_copy_byte(); // CALL 0x00d921c0
                    self.eax = i32::from(self.e.mem.i8(self.eax)) as u32; // MOVSX EAX,byte ptr [EAX]
                    self.esp = self.add32(self.esp, 0xc); // ADD ESP,0xc
                    self.cmp32(self.eax, 0x8); // CMP EAX,0x8
                    if self.zf {
                        // JZ 0x00d8c794
                        block = 0x00d8_c794;
                        continue;
                    }
                    block = 0x00d8_c968;
                }
                0x00d8_c968 => {
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_c96d => {
                    self.eax = self.esi.wrapping_add(0x34); // LEA EAX,[ESI + 0x34]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x10), self.eax); // MOV dword ptr [ESP + 0x10],EAX
                    block = 0x00d8_c974;
                }
                0x00d8_c974 => {
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x10)); // MOV EDX,dword ptr [ESP + 0x10]
                    self.fld32(self.edx.wrapping_add(0x4)); // FLD float ptr [EDX + 0x4]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.fmul32(self.esp.wrapping_add(0x78)); // FMUL float ptr [ESP + 0x78]
                    self.ecx = self.edx.wrapping_sub(0x24); // LEA ECX,[EDX + -0x24]
                    self.edx = self.add32(self.edx, 0xffffffec); // ADD EDX,-0x14
                    self.fadd32(self.edi.wrapping_add(0x8)); // FADD float ptr [EDI + 0x8]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fst32(self.esp.wrapping_add(0x2c)); // FST float ptr [ESP + 0x2c]
                    self.fstp32(self.edi.wrapping_add(0x8)); // FSTP float ptr [EDI + 0x8]
                    self.xmm[4] = movss_load(self.e.mem.u32(self.edx.wrapping_add(0x20))); // MOVSS XMM4,dword ptr [EDX + 0x20]
                    self.fld32(self.edx.wrapping_sub(0x14)); // FLD float ptr [EDX + -0x14]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x40)); // MOVAPS XMM1,xmmword ptr [EBX + 0x40]
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [EBX + 0x10]
                    self.fadd32(self.esp.wrapping_add(0x2c)); // FADD float ptr [ESP + 0x2c]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x50)); // MOVAPS XMM2,xmmword ptr [EBX + 0x50]
                    self.xmm[3] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [EAX + 0x10]
                    self.xmm[5] = load_vec(self.e, self.eax.wrapping_add(0x50)); // MOVAPS XMM5,xmmword ptr [EAX + 0x50]
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.fld32(self.edx.wrapping_add(0x1c)); // FLD float ptr [EDX + 0x1c]
                    self.xmm[0] = sub(self.xmm[0], self.xmm[1]); // SUBPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EBX + 0x20]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[2]); // SUBPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x40)); // MOVAPS XMM2,xmmword ptr [EAX + 0x40]
                    self.xmm[3] = sub(self.xmm[3], self.xmm[2]); // SUBPS XMM3,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [EAX + 0x20]
                    self.xmm[0] = sub(self.xmm[0], self.xmm[3]); // SUBPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.ecx); // MOVAPS XMM3,xmmword ptr [ECX]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = load_vec(self.e, self.edx); // MOVAPS XMM3,xmmword ptr [EDX]
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x40)); // MOV EDX,dword ptr [EBX + 0x40]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[5]); // SUBPS XMM2,XMM5
                    self.xmm[2] = mul(self.xmm[2], self.xmm[3]); // MULPS XMM2,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi); // MOVAPS XMM3,xmmword ptr [ESI]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x130), self.edx); // MOV dword ptr [ESP + 0x130],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x44)); // MOV EDX,dword ptr [EBX + 0x44]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x134), self.edx); // MOV dword ptr [ESP + 0x134],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x48)); // MOV EDX,dword ptr [EBX + 0x48]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x138), self.edx); // MOV dword ptr [ESP + 0x138],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x4c)); // MOV EDX,dword ptr [EBX + 0x4c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x13c), self.edx); // MOV dword ptr [ESP + 0x13c],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x50)); // MOV EDX,dword ptr [EBX + 0x50]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x170), self.edx); // MOV dword ptr [ESP + 0x170],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x54)); // MOV EDX,dword ptr [EBX + 0x54]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x174), self.edx); // MOV dword ptr [ESP + 0x174],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x58)); // MOV EDX,dword ptr [EBX + 0x58]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x178), self.edx); // MOV dword ptr [ESP + 0x178],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x5c)); // MOV EDX,dword ptr [EBX + 0x5c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x17c), self.edx); // MOV dword ptr [ESP + 0x17c],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x40)); // MOV EDX,dword ptr [EAX + 0x40]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x150), self.edx); // MOV dword ptr [ESP + 0x150],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x44)); // MOV EDX,dword ptr [EAX + 0x44]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x154), self.edx); // MOV dword ptr [ESP + 0x154],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x48)); // MOV EDX,dword ptr [EAX + 0x48]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x158), self.edx); // MOV dword ptr [ESP + 0x158],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x4c)); // MOV EDX,dword ptr [EAX + 0x4c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x15c), self.edx); // MOV dword ptr [ESP + 0x15c],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x50)); // MOV EDX,dword ptr [EAX + 0x50]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x190), self.edx); // MOV dword ptr [ESP + 0x190],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x54)); // MOV EDX,dword ptr [EAX + 0x54]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x194), self.edx); // MOV dword ptr [ESP + 0x194],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x58)); // MOV EDX,dword ptr [EAX + 0x58]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esp.wrapping_add(0x170)); // MOVAPS XMM3,xmmword ptr [ESP + 0x170]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x198), self.edx); // MOV dword ptr [ESP + 0x198],EDX
                    self.eax = self.e.mem.u32(self.eax.wrapping_add(0x5c)); // MOV EAX,dword ptr [EAX + 0x5c]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x10)); // MOV EDX,dword ptr [ESP + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[1]); // ADDPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, self.esp.wrapping_add(0x130)); // MOVAPS XMM1,xmmword ptr [ESP + 0x130]
                    self.xmm[2] = add(self.xmm[2], self.xmm[0]); // ADDPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0x150)); // MOVAPS XMM0,xmmword ptr [ESP + 0x150]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x19c), self.eax); // MOV dword ptr [ESP + 0x19c],EAX
                    self.xmm[5] = load_vec(self.e, self.edx.wrapping_sub(0x14)); // MOVAPS XMM5,xmmword ptr [EDX + -0x14]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.ecx); // MOVAPS XMM0,xmmword ptr [ECX]
                    self.eax = self.edx.wrapping_sub(0x14); // LEA EAX,[EDX + -0x14]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0x190)); // MOVAPS XMM0,xmmword ptr [ESP + 0x190]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x50), self.eax); // MOV dword ptr [ESP + 0x50],EAX
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.eax = self.e.mem.u32(self.ebp.wrapping_add(0x8)); // MOV EAX,dword ptr [EBP + 0x8]
                    self.x87.fld_st(0); // FLD ST0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[5]); // MULPS XMM0,XMM5
                    self.fmul32(self.eax.wrapping_add(0x40)); // FMUL float ptr [EAX + 0x40]
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[5]); // MULPS XMM1,XMM5
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x2c))); // MOVSS XMM1,dword ptr [ESP + 0x2c]
                    self.fmul32(self.esp.wrapping_add(0x30)); // FMUL float ptr [ESP + 0x30]
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0x0); // SHUFPS XMM1,XMM1,0x0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0][0] = self.xmm[4][0]; // MOVSS XMM0,XMM4
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0x0); // SHUFPS XMM0,XMM0,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xaa); // SHUFPS XMM2,XMM0,0xaa
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[1]); // ADDSS XMM2,XMM1
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x7c), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0x7c],XMM2
                    self.fsub32(self.esp.wrapping_add(0x7c)); // FSUB float ptr [ESP + 0x7c]
                    self.eax = self.edx.wrapping_sub(0x4); // LEA EAX,[EDX + -0x4]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x2c))); // MOVSS XMM0,dword ptr [ESP + 0x2c]
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xc0), self.xmm[0][0].to_bits()); // MOVSS dword ptr [ESP + 0xc0],XMM0
                    self.fld32(self.edx.wrapping_sub(0x18)); // FLD float ptr [EDX + -0x18]
                    self.fmul32(self.esp.wrapping_add(0xc0)); // FMUL float ptr [ESP + 0xc0]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x2c), self.eax); // MOV dword ptr [ESP + 0x2c],EAX
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.fld32(self.esp.wrapping_add(0x30)); // FLD float ptr [ESP + 0x30]
                    self.fld32(self.eax); // FLD float ptr [EAX]
                    self.fcom_st(1, true); // FCOMP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d8cb77
                        block = 0x00d8_cb77;
                        continue;
                    }
                    block = 0x00d8_cb66;
                }
                0x00d8_cb66 => {
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x2c)); // MOV EAX,dword ptr [ESP + 0x2c]
                    self.fdivr32(self.eax); // FDIVR float ptr [EAX]
                    self.fstp32(self.edi.wrapping_add(0x4)); // FSTP float ptr [EDI + 0x4]
                    self.fld32(self.eax); // FLD float ptr [EAX]
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    block = 0x00d8_cb96; // JMP 0x00d8cb96
                }
                0x00d8_cb77 => {
                    self.fld32(self.edx); // FLD float ptr [EDX]
                    self.fcom_st(1, true); // FCOMP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x41); // TEST AH,0x41
                    if !self.zf {
                        // JNZ 0x00d8cb8f
                        block = 0x00d8_cb8f;
                        continue;
                    }
                    block = 0x00d8_cb82;
                }
                0x00d8_cb82 => {
                    self.fdivr32(self.edx); // FDIVR float ptr [EDX]
                    self.fstp32(self.edi.wrapping_add(0x4)); // FSTP float ptr [EDI + 0x4]
                    self.fld32(self.edx); // FLD float ptr [EDX]
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    block = 0x00d8_cb96; // JMP 0x00d8cb96
                }
                0x00d8_cb8f => {
                    self.x87.pop(); // FSTP ST0
                    self.x87.push(1.0); // FLD1
                    self.fstp32(self.edi.wrapping_add(0x4)); // FSTP float ptr [EDI + 0x4]
                    block = 0x00d8_cb96;
                }
                0x00d8_cb96 => {
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x30))); // MOVSS XMM0,dword ptr [ESP + 0x30]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[1] = self.xmm[4]; // MOVAPS XMM1,XMM4
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[4], 0x0); // SHUFPS XMM1,XMM4,0x0
                    self.xmm[0] = self.xmm[1]; // MOVAPS XMM0,XMM1
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EAX + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[5]); // ADDPS XMM2,XMM5
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.eax.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EAX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx); // MOVAPS XMM2,xmmword ptr [ECX]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x50)); // MOV ECX,dword ptr [ESP + 0x50]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx); // MOVAPS XMM2,xmmword ptr [ECX]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x20],XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EAX + 0x20]
                    self.xmm[1] = add(self.xmm[1], self.xmm[0]); // ADDPS XMM1,XMM0
                    store_vec(self.e, self.eax.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [EAX + 0x20],XMM1
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[4]); // ADDSS XMM2,XMM4
                    self.push(self.esi); // PUSH ESI
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x14)); // MOV EDX,dword ptr [ESP + 0x14]
                    self.edx = self.add32(self.edx, 0x1c); // ADD EDX,0x1c
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.push(self.edx); // PUSH EDX
                    self.eax = self.esp.wrapping_add(0x76); // LEA EAX,[ESP + 0x76]
                    self.push(self.eax); // PUSH EAX
                    self.call_copy_byte(); // CALL 0x00d921c0
                    self.eax = i32::from(self.e.mem.i8(self.eax)) as u32; // MOVSX EAX,byte ptr [EAX]
                    let at = self.esp.wrapping_add(0x1c); // ADD dword ptr [ESP + 0x1c],0x50
                    let value = self.add32(self.e.mem.u32(at), 0x50);
                    self.e.mem.set_u32(at, value);
                    self.esp = self.add32(self.esp, 0xc); // ADD ESP,0xc
                    self.esi = self.add32(self.esi, 0x50); // ADD ESI,0x50
                    self.cmp32(self.eax, 0xa); // CMP EAX,0xa
                    if self.zf {
                        // JZ 0x00d8c974
                        block = 0x00d8_c974;
                        continue;
                    }
                    block = 0x00d8_cc4c;
                }
                0x00d8_cc4c => {
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_cc51 => {
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [EBX + 0x10]
                    self.xmm[1] = load_vec(self.e, self.esi); // MOVAPS XMM1,xmmword ptr [ESI]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[4] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EAX + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [EAX + 0x10]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [ESI + 0x30]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[2]); // SUBPS XMM1,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[4]); // MULPS XMM0,XMM4
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xaa); // SHUFPS XMM2,XMM0,0xaa
                    self.ecx = self.esp.wrapping_add(0x270); // LEA ECX,[ESP + 0x270]
                    self.push(self.ecx); // PUSH ECX
                    self.edx = self.esp.wrapping_add(0x214); // LEA EDX,[ESP + 0x214]
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[1]); // ADDSS XMM2,XMM1
                    store_vec(self.e, self.esp.wrapping_add(0x274), self.xmm[2]); // MOVAPS xmmword ptr [ESP + 0x274],XMM2
                    self.xmm[0] = load_vec(self.e, self.esi); // MOVAPS XMM0,xmmword ptr [ESI]
                    self.push(self.edx); // PUSH EDX
                    self.eax = self.esp.wrapping_add(0x238); // LEA EAX,[ESP + 0x238]
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xff); // SHUFPS XMM0,XMM0,0xff
                    self.push(self.eax); // PUSH EAX
                    store_vec(self.e, self.esp.wrapping_add(0x21c), self.xmm[0]); // MOVAPS xmmword ptr [ESP + 0x21c],XMM0
                    self.call_cdecl_returning_first(VECTOR_SUBTRACT_FIRST_LANE, 3); // CALL 0x00c6ccc0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.ecx = self.esp.wrapping_add(0x1ec); // LEA ECX,[ESP + 0x1ec]
                    self.push(self.ecx); // PUSH ECX
                    self.edx = self.esp.wrapping_add(0x240); // LEA EDX,[ESP + 0x240]
                    self.push(self.edx); // PUSH EDX
                    self.eax = self.esp.wrapping_add(0x1e4); // LEA EAX,[ESP + 0x1e4]
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xff); // SHUFPS XMM0,XMM0,0xff
                    self.push(self.eax); // PUSH EAX
                    store_vec(self.e, self.esp.wrapping_add(0x1f8), self.xmm[0]); // MOVAPS xmmword ptr [ESP + 0x1f8],XMM0
                    self.call_cdecl_returning_first(VECTOR_MULTIPLY_FIRST_LANE, 3); // CALL 0x00891060
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0x1e8)); // MOVAPS XMM0,xmmword ptr [ESP + 0x1e8]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x64)); // MOV EAX,dword ptr [ESP + 0x64]
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [EAX + 0x30]
                    self.xmm[4] = load_vec(self.e, self.esi); // MOVAPS XMM4,xmmword ptr [ESI]
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0x0); // SHUFPS XMM0,XMM0,0x0
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[4] = mul(self.xmm[4], self.xmm[2]); // MULPS XMM4,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[3]); // MULPS XMM2,XMM3
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = add(self.xmm[3], self.xmm[4]); // ADDPS XMM3,XMM4
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[3]); // MOVAPS xmmword ptr [EBX + 0x10],XMM3
                    self.xmm[3] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [EAX + 0x10]
                    self.xmm[3] = sub(self.xmm[3], self.xmm[2]); // SUBPS XMM3,XMM2
                    store_vec(self.e, self.eax.wrapping_add(0x10), self.xmm[3]); // MOVAPS xmmword ptr [EAX + 0x10],XMM3
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ESI + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [ESI + 0x30]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM1,dword ptr [EDI]
                    self.ecx = self.esp.wrapping_add(0x1e8); // LEA ECX,[ESP + 0x1e8]
                    self.push(self.ecx); // PUSH ECX
                    self.edx = self.esp.wrapping_add(0xec); // LEA EDX,[ESP + 0xec]
                    self.push(self.edx); // PUSH EDX
                    store_vec(self.e, self.esp.wrapping_add(0xf0), self.xmm[1]); // MOVAPS xmmword ptr [ESP + 0xf0],XMM1
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EBX + 0x20]
                    self.xmm[1] = add(self.xmm[1], self.xmm[2]); // ADDPS XMM1,XMM2
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [EBX + 0x20],XMM1
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    self.xmm[1] = add(self.xmm[1], self.xmm[0]); // ADDPS XMM1,XMM0
                    store_vec(self.e, self.eax.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [EAX + 0x20],XMM1
                    self.eax = self.esp.wrapping_add(0x2a0); // LEA EAX,[ESP + 0x2a0]
                    self.push(self.eax); // PUSH EAX
                    self.call_cdecl_returning_first(VECTOR_ADD_FIRST_LANE, 3); // CALL 0x00c6cce0
                    self.ecx = self.e.mem.u32(self.eax); // MOV ECX,dword ptr [EAX]
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x4)); // MOV EDX,dword ptr [EAX + 0x4]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xf4), self.ecx); // MOV dword ptr [ESP + 0xf4],ECX
                    self.ecx = self.e.mem.u32(self.eax.wrapping_add(0x8)); // MOV ECX,dword ptr [EAX + 0x8]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xf8), self.edx); // MOV dword ptr [ESP + 0xf8],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0xc)); // MOV EDX,dword ptr [EAX + 0xc]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xfc), self.ecx); // MOV dword ptr [ESP + 0xfc],ECX
                    self.e.mem.set_u32(self.esp.wrapping_add(0x100), self.edx); // MOV dword ptr [ESP + 0x100],EDX
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0xf4)); // MOVAPS XMM0,xmmword ptr [ESP + 0xf4]
                    self.e.mem.set_u32(self.edi, self.xmm[0][0].to_bits()); // MOVSS dword ptr [EDI],XMM0
                    self.esp = self.add32(self.esp, 0x24); // ADD ESP,0x24
                    self.edi = self.add32(self.edi, 0x4); // ADD EDI,0x4
                    self.esi = self.add32(self.esi, 0x40); // ADD ESI,0x40
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_cde0 => {
                    self.xmm[0] = load_vec(self.e, self.esi); // MOVAPS XMM0,xmmword ptr [ESI]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    store_vec(self.e, self.ebx.wrapping_add(0x30), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x30],XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    store_vec(self.e, self.eax.wrapping_add(0x30), self.xmm[0]); // MOVAPS xmmword ptr [EAX + 0x30],XMM0
                    self.esi = self.add32(self.esi, 0x20); // ADD ESI,0x20
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_cdfb => {
                    self.ecx = self.e.mem.u32(self.esi.wrapping_add(0x10)); // MOV ECX,dword ptr [ESI + 0x10]
                    self.eax = self.ebx; // MOV EAX,EBX
                    self.eax = self.xor32(self.eax, self.e.mem.u32(self.esp.wrapping_add(0x4c))); // XOR EAX,dword ptr [ESP + 0x4c]
                    self.ecx = self.neg32(self.ecx); // NEG ECX
                    self.eax = self.and32(self.eax, self.ecx); // AND EAX,ECX
                    self.ecx = self.e.mem.u32(self.ebp.wrapping_add(0x8)); // MOV ECX,dword ptr [EBP + 0x8]
                    self.fld32(self.ecx.wrapping_add(0x30)); // FLD float ptr [ECX + 0x30]
                    self.push(self.ecx); // PUSH ECX
                    self.fimul32(self.esp.wrapping_add(0x78)); // FIMUL dword ptr [ESP + 0x78]
                    self.eax = self.xor32(self.eax, self.ebx); // XOR EAX,EBX
                    self.ecx = self.esp.wrapping_add(0x1f4); // LEA ECX,[ESP + 0x1f4]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x14), self.eax); // MOV dword ptr [ESP + 0x14],EAX
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.fld32(self.esp.wrapping_add(0x30)); // FLD float ptr [ESP + 0x30]
                    self.fstp32(self.esp); // FSTP float ptr [ESP]
                    self.call_this(VECTOR_FROM_FLOAT, 1); // CALL 0x005dbf20
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x10)); // MOV EAX,dword ptr [ESP + 0x10]
                    self.xmm[1] = load_vec(self.e, self.esi); // MOVAPS XMM1,xmmword ptr [ESI]
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0x1f0)); // MOVAPS XMM0,xmmword ptr [ESP + 0x1f0]
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[1]); // ADDPS XMM2,XMM1
                    store_vec(self.e, self.eax.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EAX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0x0); // SHUFPS XMM0,XMM0,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = add(self.xmm[1], self.xmm[2]); // ADDPS XMM1,XMM2
                    store_vec(self.e, self.eax.wrapping_add(0x10), self.xmm[1]); // MOVAPS xmmword ptr [EAX + 0x10],XMM1
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x40)); // MOVAPS XMM1,xmmword ptr [EAX + 0x40]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.eax.wrapping_add(0x40), self.xmm[0]); // MOVAPS xmmword ptr [EAX + 0x40],XMM0
                    self.esi = self.add32(self.esi, 0x20); // ADD ESI,0x20
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_ce72 => {
                    self.edx = self.ebx.wrapping_add(0x20); // LEA EDX,[EBX + 0x20]
                    self.push(self.edx); // PUSH EDX
                    self.push(self.esi); // PUSH ESI
                    self.ecx = self.esp.wrapping_add(0x208); // LEA ECX,[ESP + 0x208]
                    self.call_this(SET_MUL3, 2); // CALL 0x00c7f750
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.ecx = self.add32(self.ecx, 0x20); // ADD ECX,0x20
                    self.push(self.ecx); // PUSH ECX
                    self.eax = self.esi.wrapping_add(0x30); // LEA EAX,[ESI + 0x30]
                    self.push(self.eax); // PUSH EAX
                    self.ecx = self.esp.wrapping_add(0x248); // LEA ECX,[ESP + 0x248]
                    self.call_this(SET_MUL3, 2); // CALL 0x00c7f750
                    self.edx = self.ebx.wrapping_add(0x50); // LEA EDX,[EBX + 0x50]
                    self.push(self.edx); // PUSH EDX
                    self.push(self.esi); // PUSH ESI
                    self.ecx = self.esp.wrapping_add(0x228); // LEA ECX,[ESP + 0x228]
                    self.call_this(SET_MUL3, 2); // CALL 0x00c7f750
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.eax = self.add32(self.eax, 0x50); // ADD EAX,0x50
                    self.push(self.eax); // PUSH EAX
                    self.eax = self.esi.wrapping_add(0x30); // LEA EAX,[ESI + 0x30]
                    self.push(self.eax); // PUSH EAX
                    self.ecx = self.esp.wrapping_add(0x268); // LEA ECX,[ESP + 0x268]
                    self.call_this(SET_MUL3, 2); // CALL 0x00c7f750
                    self.xmm[1] = load_vec(self.e, self.esp.wrapping_add(0x200)); // MOVAPS XMM1,xmmword ptr [ESP + 0x200]
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [EBX + 0x10]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x10],XMM0
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x40)); // MOVAPS XMM1,xmmword ptr [EBX + 0x40]
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0x220)); // MOVAPS XMM0,xmmword ptr [ESP + 0x220]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.ebx.wrapping_add(0x40), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x40],XMM0
                    self.xmm[1] = load_vec(self.e, self.esp.wrapping_add(0x240)); // MOVAPS XMM1,xmmword ptr [ESP + 0x240]
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [EAX + 0x10]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.eax.wrapping_add(0x10), self.xmm[0]); // MOVAPS xmmword ptr [EAX + 0x10],XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x40)); // MOVAPS XMM0,xmmword ptr [EAX + 0x40]
                    self.xmm[1] = load_vec(self.e, self.esp.wrapping_add(0x260)); // MOVAPS XMM1,xmmword ptr [ESP + 0x260]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.eax.wrapping_add(0x40), self.xmm[0]); // MOVAPS xmmword ptr [EAX + 0x40],XMM0
                    self.esi = self.add32(self.esi, 0x60); // ADD ESI,0x60
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_cf1c => {
                    self.ecx = self.e.mem.u32(self.ebp.wrapping_add(0x10)); // MOV ECX,dword ptr [EBP + 0x10]
                    self.eax = self.e.mem.u32(self.ebp.wrapping_add(0x8)); // MOV EAX,dword ptr [EBP + 0x8]
                    self.push(self.edi); // PUSH EDI
                    self.push(self.esi); // PUSH ESI
                    self.push(self.ecx); // PUSH ECX
                    self.edx = self.esp.wrapping_add(0x80); // LEA EDX,[ESP + 0x80]
                    self.push(self.edx); // PUSH EDX
                    self.push(self.eax); // PUSH EAX
                    self.call_stiff_spring_chain(); // CALL 0x00d893a0
                    self.ecx = u32::from(self.e.mem.u16(self.esi.wrapping_add(0x2))); // MOVZX ECX,word ptr [ESI + 0x2]
                    self.esp = self.add32(self.esp, 0x14); // ADD ESP,0x14
                    self.esi = self.add32(self.esi, self.e.mem.u32(self.esi.wrapping_add(0x4))); // ADD ESI,dword ptr [ESI + 0x4]
                    self.edi = self.edi.wrapping_add(self.ecx.wrapping_mul(4)); // LEA EDI,[EDI + ECX*0x4]
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_cf45 => {
                    self.edx = self.e.mem.u32(self.ebp.wrapping_add(0x10)); // MOV EDX,dword ptr [EBP + 0x10]
                    self.ecx = self.e.mem.u32(self.ebp.wrapping_add(0x8)); // MOV ECX,dword ptr [EBP + 0x8]
                    self.push(self.edi); // PUSH EDI
                    self.push(self.esi); // PUSH ESI
                    self.push(self.edx); // PUSH EDX
                    self.eax = self.esp.wrapping_add(0x80); // LEA EAX,[ESP + 0x80]
                    self.push(self.eax); // PUSH EAX
                    self.push(self.ecx); // PUSH ECX
                    self.call_ball_and_socket_chain(); // CALL 0x00d89690
                    self.eax = u32::from(self.e.mem.u16(self.esi.wrapping_add(0x2))); // MOVZX EAX,word ptr [ESI + 0x2]
                    self.edx = self.eax.wrapping_add(self.eax.wrapping_mul(2)); // LEA EDX,[EAX + EAX*0x2]
                    self.esp = self.add32(self.esp, 0x14); // ADD ESP,0x14
                    self.edi = self.edi.wrapping_add(self.edx.wrapping_mul(4)); // LEA EDI,[EDI + EDX*0x4]
                    block = 0x00d8_cf69;
                }
                0x00d8_cf69 => {
                    self.esi = self.add32(self.esi, self.e.mem.u32(self.esi.wrapping_add(0x4))); // ADD ESI,dword ptr [ESI + 0x4]
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_cf71 => {
                    self.eax = self.e.mem.u32(self.ebp.wrapping_add(0x10)); // MOV EAX,dword ptr [EBP + 0x10]
                    self.edx = self.e.mem.u32(self.ebp.wrapping_add(0x8)); // MOV EDX,dword ptr [EBP + 0x8]
                    self.push(self.edi); // PUSH EDI
                    self.push(self.esi); // PUSH ESI
                    self.push(self.eax); // PUSH EAX
                    self.ecx = self.esp.wrapping_add(0x80); // LEA ECX,[ESP + 0x80]
                    self.push(self.ecx); // PUSH ECX
                    self.push(self.edx); // PUSH EDX
                    self.call_powered_chain(); // CALL 0x00d8a3a0
                    self.eax = u32::from(self.e.mem.u16(self.esi.wrapping_add(0x4))); // MOVZX EAX,word ptr [ESI + 0x4]
                    self.eax = self.eax.wrapping_add(self.eax.wrapping_mul(2)); // LEA EAX,[EAX + EAX*0x2]
                    self.esp = self.add32(self.esp, 0x14); // ADD ESP,0x14
                    self.esi = self.add32(self.esi, self.e.mem.u32(self.esi.wrapping_add(0x10))); // ADD ESI,dword ptr [ESI + 0x10]
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(8)); // LEA EDI,[EDI + EAX*0x8]
                    block = 0x00d8_ab0c; // JMP 0x00d8ab0c
                }
                0x00d8_cf9d => {
                    panic!("INT3 at 00d8cf9d"); // INT3
                }
                0x00d8_cf9e => {
                    self.edi = self.pop(); // POP EDI
                    self.esi = self.pop(); // POP ESI
                    self.ebx = self.pop(); // POP EBX
                    self.esp = self.ebp; // MOV ESP,EBP
                    self.ebp = self.pop(); // POP EBP
                    return self.eax; // RET
                }
                _ => unreachable!("no block starts at {block:08x}"),
            }
        }
    }
}

/// The switch table at `00d92138`: the block each record type jumps to.
const BY_THE_STEPS_SWITCH: [u32; 32] = [
    0x00d9_212a,
    0x00d8_fbe0,
    0x00d9_20f5,
    0x00d8_fbf9,
    0x00d9_2129,
    0x00d9_1320,
    0x00d9_1430,
    0x00d9_1560,
    0x00d9_1920,
    0x00d9_17c9,
    0x00d9_1af9,
    0x00d9_1ddd,
    0x00d9_1271,
    0x00d9_0e30,
    0x00d9_0d29,
    0x00d9_1058,
    0x00d9_01dd,
    0x00d9_01dd,
    0x00d9_0304,
    0x00d9_0304,
    0x00d8_fc0f,
    0x00d8_fc0f,
    0x00d9_0abc,
    0x00d9_076a,
    0x00d9_0598,
    0x00d9_1f6c,
    0x00d9_1f87,
    0x00d9_1ffe,
    0x00d9_2129,
    0x00d9_20a8,
    0x00d9_20d1,
    0x00d9_20fd,
];

impl SolverMachine<'_> {
    /// The blocks of the function that starts at `0x00d8_fb70`, one arm per block.
    fn run_by_the_steps(&mut self) -> u32 {
        let mut block: u32 = 0x00d8_fb70;
        loop {
            match block {
                0x00d8_fb70 => {
                    self.push(self.ebp); // PUSH EBP
                    self.ebp = self.esp; // MOV EBP,ESP
                    self.esp = self.and32(self.esp, 0xfffffff0); // AND ESP,0xfffffff0
                    self.esp = self.sub32(self.esp, 0x284); // SUB ESP,0x284
                    self.cmp8(self.e.mem.u8(LICENSE_VALID_FLAG), 0x0); // CMP byte ptr [0x01269204],0x0
                    self.push(self.ebx); // PUSH EBX
                    self.push(self.esi); // PUSH ESI
                    self.push(self.edi); // PUSH EDI
                    if !self.zf {
                        // JNZ 0x00d8fba7
                        block = 0x00d8_fba7;
                        continue;
                    }
                    block = 0x00d8_fb88;
                }
                0x00d8_fb88 => {
                    self.push(LICENSE_VALID_FLAG); // PUSH 0x1269204
                    self.call_cdecl(LICENSE_CHECK, 1); // CALL 0x00df22f0
                    self.esp = self.add32(self.esp, 0x4); // ADD ESP,0x4
                    self.cmp8(self.e.mem.u8(LICENSE_VALID_FLAG), 0x0); // CMP byte ptr [0x01269204],0x0
                    if !self.zf {
                        // JNZ 0x00d8fba7
                        block = 0x00d8_fba7;
                        continue;
                    }
                    block = 0x00d8_fb9e;
                }
                0x00d8_fb9e => {
                    self.eax = self.xor32(self.eax, self.eax); // XOR EAX,EAX
                    self.edi = self.pop(); // POP EDI
                    self.esi = self.pop(); // POP ESI
                    self.ebx = self.pop(); // POP EBX
                    self.esp = self.ebp; // MOV ESP,EBP
                    self.ebp = self.pop(); // POP EBP
                    return self.eax; // RET
                }
                0x00d8_fba7 => {
                    self.cmp32(self.e.mem.u32(self.ebp.wrapping_add(0x10)), 0x0); // CMP dword ptr [EBP + 0x10],0x0
                    self.eax = self.e.mem.u32(self.ebp.wrapping_add(0xc)); // MOV EAX,dword ptr [EBP + 0xc]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x74), self.eax); // MOV dword ptr [ESP + 0x74],EAX
                    if self.zf {
                        // JZ 0x00d8fbb8
                        block = 0x00d8_fbb8;
                        continue;
                    }
                    block = 0x00d8_fbb4;
                }
                0x00d8_fbb4 => {
                    self.x87.push(0.0); // FLDZ
                    block = 0x00d8_fbba; // JMP 0x00d8fbba
                }
                0x00d8_fbb8 => {
                    self.x87.push(1.0); // FLD1
                    block = 0x00d8_fbba;
                }
                0x00d8_fbba => {
                    self.esi = self.e.mem.u32(self.ebp.wrapping_add(0x14)); // MOV ESI,dword ptr [EBP + 0x14]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.eax = (self.eax & 0xffffff00) | u32::from(self.e.mem.u8(self.esi)); // MOV AL,byte ptr [ESI]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.edi = self.e.mem.u32(self.ebp.wrapping_add(0x1c)); // MOV EDI,dword ptr [EBP + 0x1c]
                    self.fstp32(self.esp.wrapping_add(0x78)); // FSTP float ptr [ESP + 0x78]
                    self.test8(self.eax as u8, self.eax as u8); // TEST AL,AL
                    if self.zf {
                        // JZ 0x00d9212a
                        block = 0x00d9_212a;
                        continue;
                    }
                    block = 0x00d8_fbd6;
                }
                0x00d8_fbd6 => {
                    block = 0x00d8_fbe0; // JMP 0x00d8fbe0
                }
                0x00d8_fbe0 => {
                    self.ebx = u32::from(self.e.mem.u16(self.esi.wrapping_add(0x4))); // MOVZX EBX,word ptr [ESI + 0x4]
                    self.eax = u32::from(self.e.mem.u16(self.esi.wrapping_add(0x6))); // MOVZX EAX,word ptr [ESI + 0x6]
                    self.ecx = self.e.mem.u32(self.ebp.wrapping_add(0x18)); // MOV ECX,dword ptr [EBP + 0x18]
                    self.ebx = self.shl32(self.ebx, 7); // SHL EBX,0x7
                    self.eax = self.shl32(self.eax, 7); // SHL EAX,0x7
                    self.ebx = self.add32(self.ebx, self.ecx); // ADD EBX,ECX
                    self.eax = self.add32(self.eax, self.ecx); // ADD EAX,ECX
                    self.e.mem.set_u32(self.esp.wrapping_add(0x4c), self.eax); // MOV dword ptr [ESP + 0x4c],EAX
                    block = 0x00d8_fbf9;
                }
                0x00d8_fbf9 => {
                    self.esi = self.add32(self.esi, 0x10); // ADD ESI,0x10
                    block = 0x00d8_fbfc;
                }
                0x00d8_fbfc => {
                    self.eax = i32::from(self.e.mem.i8(self.esi)) as u32; // MOVSX EAX,byte ptr [ESI]
                    self.cmp32(self.eax, 0x1f); // CMP EAX,0x1f
                    if !self.cf && !self.zf {
                        // JA 0x00d92129
                        block = 0x00d9_2129;
                        continue;
                    }
                    block = 0x00d8_fc08;
                }
                0x00d8_fc08 => {
                    block = BY_THE_STEPS_SWITCH[self.eax as usize]; // JMP dword ptr [EAX*0x4 + 0xd92138]
                }
                0x00d8_fc0f => {
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EDX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = load_vec(self.e, self.edx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [EDX + 0x10]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[2] = load_vec(self.e, self.edx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [EDX + 0x20]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi); // MOVAPS XMM2,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.ecx = self.edx.wrapping_add(0x10); // LEA ECX,[EDX + 0x10]
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.edx = self.add32(self.edx, 0x20); // ADD EDX,0x20
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xcc), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0xcc],XMM2
                    self.fld32(self.esp.wrapping_add(0xcc)); // FLD float ptr [ESP + 0xcc]
                    self.fst32(self.esp.wrapping_add(0x1c0)); // FST float ptr [ESP + 0x1c0]
                    self.xmm[0] = load_vec(self.e, self.ecx); // MOVAPS XMM0,xmmword ptr [ECX]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x40)); // MOVAPS XMM3,xmmword ptr [ESI + 0x40]
                    self.xmm[2] = load_vec(self.e, self.edx); // MOVAPS XMM2,xmmword ptr [EDX]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x50)); // MOVAPS XMM0,xmmword ptr [ESI + 0x50]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [ESI + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xb8), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0xb8],XMM2
                    self.fld32(self.esp.wrapping_add(0xb8)); // FLD float ptr [ESP + 0xb8]
                    self.fst32(self.esp.wrapping_add(0x1c4)); // FST float ptr [ESP + 0x1c4]
                    self.fld32(self.esi.wrapping_add(0x2c)); // FLD float ptr [ESI + 0x2c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.x87.fld_st(1); // FLD ST1
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.x87.fld_st(1); // FLD ST1
                    self.fmul32(self.esi.wrapping_add(0x60)); // FMUL float ptr [ESI + 0x60]
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.fld32(self.esi.wrapping_add(0x5c)); // FLD float ptr [ESI + 0x5c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esi.wrapping_add(0x60)); // FLD float ptr [ESI + 0x60]
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.x87.faddp(2); // FADDP ST2,ST0
                    self.x87.fxch(1); // FXCH
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x30)); // FLD float ptr [ESP + 0x30]
                    self.fld32(self.edi); // FLD float ptr [EDI]
                    self.x87.fchs(); // FCHS
                    self.fcompp(); // FCOMPP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d8fe4e
                        block = 0x00d8_fe4e;
                        continue;
                    }
                    block = 0x00d8_fd1d;
                }
                0x00d8_fd1d => {
                    self.x87.pop(); // FSTP ST0
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fld32(self.edi.wrapping_add(0x4)); // FLD float ptr [EDI + 0x4]
                    self.x87.fchs(); // FCHS
                    self.fcompp(); // FCOMPP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d8fe6b
                        block = 0x00d8_fe6b;
                        continue;
                    }
                    block = 0x00d8_fd35;
                }
                0x00d8_fd35 => {
                    self.xmm[5] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x30))); // MOVSS XMM5,dword ptr [ESP + 0x30]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.xmm[6] = load_vec(self.e, self.esi); // MOVAPS XMM6,xmmword ptr [ESI]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.eax = self.add32(self.eax, 0x30); // ADD EAX,0x30
                    self.xmm[1] = self.xmm[5]; // MOVAPS XMM1,XMM5
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[5], 0x0); // SHUFPS XMM1,XMM5,0x0
                    self.xmm[0] = self.xmm[1]; // MOVAPS XMM0,XMM1
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax); // MOVAPS XMM2,xmmword ptr [EAX]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[6]); // MULPS XMM2,XMM6
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[6]); // MULPS XMM3,XMM6
                    self.xmm[6] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM6,xmmword ptr [EBX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[6]); // ADDPS XMM2,XMM6
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx); // MOVAPS XMM2,xmmword ptr [ECX]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx, self.xmm[2]); // MOVAPS xmmword ptr [ECX],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ESI + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x20],XMM0
                    self.xmm[0] = load_vec(self.e, self.edx); // MOVAPS XMM0,xmmword ptr [EDX]
                    self.xmm[1] = add(self.xmm[1], self.xmm[0]); // ADDPS XMM1,XMM0
                    store_vec(self.e, self.edx, self.xmm[1]); // MOVAPS xmmword ptr [EDX],XMM1
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[5]); // ADDSS XMM2,XMM5
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x50), self.eax); // MOV dword ptr [ESP + 0x50],EAX
                    self.eax = self.edi.wrapping_add(0x4); // LEA EAX,[EDI + 0x4]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.eax); // MOV dword ptr [ESP + 0x30],EAX
                    self.eax = self.esi.wrapping_add(0x30); // LEA EAX,[ESI + 0x30]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x10), self.eax); // MOV dword ptr [ESP + 0x10],EAX
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x50)); // MOV EAX,dword ptr [ESP + 0x50]
                    self.xmm[0] = self.xmm[4]; // MOVAPS XMM0,XMM4
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[4], 0x0); // SHUFPS XMM0,XMM4,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax); // MOVAPS XMM2,xmmword ptr [EAX]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x10)); // MOV EAX,dword ptr [ESP + 0x10]
                    self.xmm[5] = load_vec(self.e, self.eax); // MOVAPS XMM5,xmmword ptr [EAX]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[1], 0xff); // SHUFPS XMM2,XMM1,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[0], 0xff); // SHUFPS XMM3,XMM0,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[5]); // ADDPS XMM2,XMM5
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx); // MOVAPS XMM2,xmmword ptr [ECX]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx, self.xmm[2]); // MOVAPS xmmword ptr [ECX],XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[1]); // MULPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EAX,dword ptr [ESP + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.eax)); // MOVSS XMM0,dword ptr [EAX]
                    self.xmm[2] = add(self.xmm[2], self.xmm[3]); // ADDPS XMM2,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x20],XMM2
                    self.xmm[2] = load_vec(self.e, self.edx); // MOVAPS XMM2,xmmword ptr [EDX]
                    self.xmm[1] = add(self.xmm[1], self.xmm[2]); // ADDPS XMM1,XMM2
                    store_vec(self.e, self.edx, self.xmm[1]); // MOVAPS xmmword ptr [EDX],XMM1
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[4]); // ADDSS XMM0,XMM4
                    self.e.mem.set_u32(self.eax, self.xmm[0][0].to_bits()); // MOVSS dword ptr [EAX],XMM0
                    block = 0x00d8_ffb3; // JMP 0x00d8ffb3
                }
                0x00d8_fe4e => {
                    self.fmul32(self.esi.wrapping_add(0x4c)); // FMUL float ptr [ESI + 0x4c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fld32(self.edi.wrapping_add(0x4)); // FLD float ptr [EDI + 0x4]
                    self.x87.fchs(); // FCHS
                    self.fcompp(); // FCOMPP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x1); // TEST AH,0x1
                    if !self.zf {
                        // JNZ 0x00d8ff18
                        block = 0x00d8_ff18;
                        continue;
                    }
                    block = 0x00d8_fe6b;
                }
                0x00d8_fe6b => {
                    self.xmm[1] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x1c0))); // MOVSS XMM1,dword ptr [ESP + 0x1c0]
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EBX + 0x30]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xff); // SHUFPS XMM0,XMM0,0xff
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, VECTOR_CONSTANT_010C7160); // MOVAPS XMM1,xmmword ptr [0x010c7160]
                    self.xmm[2] = mul_ss(self.xmm[2], self.xmm[0]); // MULSS XMM2,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM0,dword ptr [EDI]
                    self.eax = self.add32(self.eax, 0x30); // ADD EAX,0x30
                    self.e.mem.set_u32(self.esp.wrapping_add(0x50), self.eax); // MOV dword ptr [ESP + 0x50],EAX
                    self.xmm[1] = sub_ss(self.xmm[1], self.xmm[0]); // SUBSS XMM1,XMM0
                    self.xmm[1] = max_ps(self.xmm[1], self.xmm[2]); // MAXPS XMM1,XMM2
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[1], 0x0); // SHUFPS XMM2,XMM1,0x0
                    self.xmm[0] = self.xmm[2]; // MOVAPS XMM0,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.eax); // MOVAPS XMM3,xmmword ptr [EAX]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[3]); // MULPS XMM2,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi); // MOVAPS XMM3,xmmword ptr [ESI]
                    self.xmm[6] = self.xmm[3]; // MOVAPS XMM6,XMM3
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[4] = shufps(self.xmm[4], self.xmm[0], 0xff); // SHUFPS XMM4,XMM0,0xff
                    self.xmm[6] = mul(self.xmm[6], self.xmm[4]); // MULPS XMM6,XMM4
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM4,xmmword ptr [EBX + 0x10]
                    self.xmm[4] = add(self.xmm[4], self.xmm[6]); // ADDPS XMM4,XMM6
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[4]); // MOVAPS xmmword ptr [EBX + 0x10],XMM4
                    self.xmm[4] = load_vec(self.e, self.ecx); // MOVAPS XMM4,xmmword ptr [ECX]
                    self.xmm[5] = self.xmm[2]; // MOVAPS XMM5,XMM2
                    self.xmm[5] = shufps(self.xmm[5], self.xmm[2], 0xff); // SHUFPS XMM5,XMM2,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[4] = sub(self.xmm[4], self.xmm[3]); // SUBPS XMM4,XMM3
                    store_vec(self.e, self.ecx, self.xmm[4]); // MOVAPS xmmword ptr [ECX],XMM4
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [ESI + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[2]); // MULPS XMM3,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.xmm[0] = add(self.xmm[0], self.xmm[4]); // ADDPS XMM0,XMM4
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x20],XMM0
                    self.xmm[0] = load_vec(self.e, self.edx); // MOVAPS XMM0,xmmword ptr [EDX]
                    self.xmm[3] = add(self.xmm[3], self.xmm[0]); // ADDPS XMM3,XMM0
                    store_vec(self.e, self.edx, self.xmm[3]); // MOVAPS xmmword ptr [EDX],XMM3
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[1]); // ADDSS XMM2,XMM1
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    block = 0x00d8_ffb3; // JMP 0x00d8ffb3
                }
                0x00d8_ff18 => {
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.eax = self.edi.wrapping_add(0x4); // LEA EAX,[EDI + 0x4]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.eax); // MOV dword ptr [ESP + 0x30],EAX
                    self.eax = self.esi.wrapping_add(0x30); // LEA EAX,[ESI + 0x30]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x10), self.eax); // MOV dword ptr [ESP + 0x10],EAX
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [EAX + 0x30]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.eax = self.add32(self.eax, 0x30); // ADD EAX,0x30
                    self.e.mem.set_u32(self.esp.wrapping_add(0x50), self.eax); // MOV dword ptr [ESP + 0x50],EAX
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x10)); // MOV EAX,dword ptr [ESP + 0x10]
                    self.xmm[3] = load_vec(self.e, self.eax); // MOVAPS XMM3,xmmword ptr [EAX]
                    self.xmm[2] = self.xmm[4]; // MOVAPS XMM2,XMM4
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[4], 0x0); // SHUFPS XMM2,XMM4,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[5] = self.xmm[0]; // MOVAPS XMM5,XMM0
                    self.xmm[5] = shufps(self.xmm[5], self.xmm[0], 0xff); // SHUFPS XMM5,XMM0,0xff
                    self.xmm[2] = self.xmm[3]; // MOVAPS XMM2,XMM3
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[5]); // ADDPS XMM2,XMM5
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx); // MOVAPS XMM2,xmmword ptr [ECX]
                    self.xmm[6] = self.xmm[1]; // MOVAPS XMM6,XMM1
                    self.xmm[6] = shufps(self.xmm[6], self.xmm[1], 0xff); // SHUFPS XMM6,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[6]); // MULPS XMM3,XMM6
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx, self.xmm[2]); // MOVAPS xmmword ptr [ECX],XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EAX + 0x20]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EAX,dword ptr [ESP + 0x30]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.eax)); // MOVSS XMM1,dword ptr [EAX]
                    self.xmm[3] = add(self.xmm[3], self.xmm[2]); // ADDPS XMM3,XMM2
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[3]); // MOVAPS xmmword ptr [EBX + 0x20],XMM3
                    self.xmm[2] = load_vec(self.e, self.edx); // MOVAPS XMM2,xmmword ptr [EDX]
                    self.xmm[2] = add(self.xmm[2], self.xmm[0]); // ADDPS XMM2,XMM0
                    store_vec(self.e, self.edx, self.xmm[2]); // MOVAPS xmmword ptr [EDX],XMM2
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[4]); // ADDSS XMM1,XMM4
                    self.e.mem.set_u32(self.eax, self.xmm[1][0].to_bits()); // MOVSS dword ptr [EAX],XMM1
                    block = 0x00d8_ffb3;
                }
                0x00d8_ffb3 => {
                    self.esi = self.add32(self.esi, 0x70); // ADD ESI,0x70
                    self.edi = self.add32(self.edi, 0x8); // ADD EDI,0x8
                    self.cmp8(self.e.mem.u8(self.esi), 0x14); // CMP byte ptr [ESI],0x14
                    if !self.zf {
                        // JNZ 0x00d8fbfc
                        block = 0x00d8_fbfc;
                        continue;
                    }
                    block = 0x00d8_ffc2;
                }
                0x00d8_ffc2 => {
                    self.xmm[0] = load_vec(self.e, self.ecx); // MOVAPS XMM0,xmmword ptr [ECX]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[2] = load_vec(self.e, self.edx); // MOVAPS XMM2,xmmword ptr [EDX]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi); // MOVAPS XMM2,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xc8), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0xc8],XMM2
                    self.fld32(self.esp.wrapping_add(0xc8)); // FLD float ptr [ESP + 0xc8]
                    self.fst32(self.esp.wrapping_add(0x1a0)); // FST float ptr [ESP + 0x1a0]
                    self.xmm[0] = load_vec(self.e, self.ecx); // MOVAPS XMM0,xmmword ptr [ECX]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x40)); // MOVAPS XMM3,xmmword ptr [ESI + 0x40]
                    self.xmm[2] = load_vec(self.e, self.edx); // MOVAPS XMM2,xmmword ptr [EDX]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x50)); // MOVAPS XMM0,xmmword ptr [ESI + 0x50]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [ESI + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x84), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0x84],XMM2
                    self.fld32(self.esp.wrapping_add(0x84)); // FLD float ptr [ESP + 0x84]
                    self.fst32(self.esp.wrapping_add(0x1a4)); // FST float ptr [ESP + 0x1a4]
                    self.fld32(self.esi.wrapping_add(0x2c)); // FLD float ptr [ESI + 0x2c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esi.wrapping_add(0x60)); // FLD float ptr [ESI + 0x60]
                    self.x87.fmul_st0(1); // FMUL ST1
                    self.x87.fld_st(2); // FLD ST2
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.fld32(self.esi.wrapping_add(0x5c)); // FLD float ptr [ESI + 0x5c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esi.wrapping_add(0x60)); // FLD float ptr [ESI + 0x60]
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.x87.faddp(2); // FADDP ST2,ST0
                    self.x87.fxch(1); // FXCH
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x30)); // FLD float ptr [ESP + 0x30]
                    self.fld32(self.edi); // FLD float ptr [EDI]
                    self.x87.fchs(); // FCHS
                    self.fcompp(); // FCOMPP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d90434
                        block = 0x00d9_0434;
                        continue;
                    }
                    block = 0x00d9_00c4;
                }
                0x00d9_00c4 => {
                    self.x87.pop(); // FSTP ST0
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fld32(self.edi.wrapping_add(0x4)); // FLD float ptr [EDI + 0x4]
                    self.x87.fchs(); // FCHS
                    self.fcompp(); // FCOMPP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d90451
                        block = 0x00d9_0451;
                        continue;
                    }
                    block = 0x00d9_00dc;
                }
                0x00d9_00dc => {
                    self.xmm[5] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x30))); // MOVSS XMM5,dword ptr [ESP + 0x30]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.xmm[6] = load_vec(self.e, self.esi); // MOVAPS XMM6,xmmword ptr [ESI]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x50)); // MOV EAX,dword ptr [ESP + 0x50]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[0] = self.xmm[5]; // MOVAPS XMM0,XMM5
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[5], 0x0); // SHUFPS XMM0,XMM5,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax); // MOVAPS XMM2,xmmword ptr [EAX]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[1], 0xff); // SHUFPS XMM2,XMM1,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[6]); // MULPS XMM2,XMM6
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[0], 0xff); // SHUFPS XMM3,XMM0,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[6]); // MULPS XMM3,XMM6
                    self.xmm[6] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM6,xmmword ptr [EBX + 0x10]
                    self.xmm[6] = add(self.xmm[6], self.xmm[2]); // ADDPS XMM6,XMM2
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[6]); // MOVAPS xmmword ptr [EBX + 0x10],XMM6
                    self.xmm[2] = load_vec(self.e, self.ecx); // MOVAPS XMM2,xmmword ptr [ECX]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx, self.xmm[2]); // MOVAPS xmmword ptr [ECX],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[1]); // MULPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [ESI + 0x20]
                    self.xmm[3] = add(self.xmm[3], self.xmm[2]); // ADDPS XMM3,XMM2
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM0,dword ptr [EDI]
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[3]); // MOVAPS xmmword ptr [EBX + 0x20],XMM3
                    self.xmm[2] = load_vec(self.e, self.edx); // MOVAPS XMM2,xmmword ptr [EDX]
                    self.xmm[1] = add(self.xmm[1], self.xmm[2]); // ADDPS XMM1,XMM2
                    store_vec(self.e, self.edx, self.xmm[1]); // MOVAPS xmmword ptr [EDX],XMM1
                    self.eax = self.edi.wrapping_add(0x4); // LEA EAX,[EDI + 0x4]
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[5]); // ADDSS XMM0,XMM5
                    self.e.mem.set_u32(self.edi, self.xmm[0][0].to_bits()); // MOVSS dword ptr [EDI],XMM0
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.eax); // MOV dword ptr [ESP + 0x30],EAX
                    self.eax = self.esi.wrapping_add(0x30); // LEA EAX,[ESI + 0x30]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x10), self.eax); // MOV dword ptr [ESP + 0x10],EAX
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x50)); // MOV EAX,dword ptr [ESP + 0x50]
                    self.xmm[0] = self.xmm[4]; // MOVAPS XMM0,XMM4
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[4], 0x0); // SHUFPS XMM0,XMM4,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax); // MOVAPS XMM2,xmmword ptr [EAX]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x10)); // MOV EAX,dword ptr [ESP + 0x10]
                    self.xmm[5] = load_vec(self.e, self.eax); // MOVAPS XMM5,xmmword ptr [EAX]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[1], 0xff); // SHUFPS XMM2,XMM1,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[0], 0xff); // SHUFPS XMM3,XMM0,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[5] = add(self.xmm[5], self.xmm[2]); // ADDPS XMM5,XMM2
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[5]); // MOVAPS xmmword ptr [EBX + 0x10],XMM5
                    self.xmm[2] = load_vec(self.e, self.ecx); // MOVAPS XMM2,xmmword ptr [ECX]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx, self.xmm[2]); // MOVAPS xmmword ptr [ECX],XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[1]); // MULPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EAX,dword ptr [ESP + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.eax)); // MOVSS XMM0,dword ptr [EAX]
                    self.xmm[3] = add(self.xmm[3], self.xmm[2]); // ADDPS XMM3,XMM2
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[3]); // MOVAPS xmmword ptr [EBX + 0x20],XMM3
                    block = 0x00d9_0573; // JMP 0x00d90573
                }
                0x00d9_01dd => {
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.edx = self.ecx.wrapping_add(0x30); // LEA EDX,[ECX + 0x30]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x50), self.edx); // MOV dword ptr [ESP + 0x50],EDX
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EDX,dword ptr [ESP + 0x4c]
                    self.ecx = self.add32(self.ecx, 0x10); // ADD ECX,0x10
                    self.edx = self.add32(self.edx, 0x20); // ADD EDX,0x20
                    self.eax = self.ebx.wrapping_add(0x10); // LEA EAX,[EBX + 0x10]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x10), self.ecx); // MOV dword ptr [ESP + 0x10],ECX
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.edx); // MOV dword ptr [ESP + 0x30],EDX
                                                                               // LEA ECX,[ECX]: no operation
                    block = 0x00d9_0200;
                }
                0x00d9_0200 => {
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x10)); // MOV EDX,dword ptr [ESP + 0x10]
                    self.xmm[0] = load_vec(self.e, self.edx); // MOVAPS XMM0,xmmword ptr [EDX]
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EDX,dword ptr [ESP + 0x30]
                    self.xmm[3] = load_vec(self.e, self.edx); // MOVAPS XMM3,xmmword ptr [EDX]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi); // MOVAPS XMM3,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x50)); // MOV EDX,dword ptr [ESP + 0x50]
                    self.xmm[4] = mul(self.xmm[4], self.xmm[2]); // MULPS XMM4,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[4]); // ADDPS XMM0,XMM4
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[3], 0xff); // SHUFPS XMM3,XMM3,0xff
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[3] = sub_ss(self.xmm[3], self.xmm[0]); // SUBSS XMM3,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM0,dword ptr [EDI]
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[3] = mul_ss(self.xmm[3], self.xmm[2]); // MULSS XMM3,XMM2
                    self.esi = self.add32(self.esi, 0x30); // ADD ESI,0x30
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, VECTOR_CONSTANT_010C7160); // MOVAPS XMM0,xmmword ptr [0x010c7160]
                    self.xmm[0] = sub_ss(self.xmm[0], self.xmm[1]); // SUBSS XMM0,XMM1
                    self.xmm[0] = max_ps(self.xmm[0], self.xmm[3]); // MAXPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EBX + 0x30]
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x0); // SHUFPS XMM1,XMM0,0x0
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[2] = mul(self.xmm[2], self.xmm[3]); // MULPS XMM2,XMM3
                    self.xmm[3] = load_vec(self.e, self.edx); // MOVAPS XMM3,xmmword ptr [EDX]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = self.xmm[2]; // MOVAPS XMM3,XMM2
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[2], 0xff); // SHUFPS XMM3,XMM2,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[4] = self.xmm[1]; // MOVAPS XMM4,XMM1
                    self.xmm[4] = shufps(self.xmm[4], self.xmm[1], 0xff); // SHUFPS XMM4,XMM1,0xff
                    self.xmm[4] = mul(self.xmm[4], self.xmm[5]); // MULPS XMM4,XMM5
                    self.xmm[5] = load_vec(self.e, self.eax); // MOVAPS XMM5,xmmword ptr [EAX]
                    self.xmm[3] = add(self.xmm[3], self.xmm[5]); // ADDPS XMM3,XMM5
                    store_vec(self.e, self.eax, self.xmm[3]); // MOVAPS xmmword ptr [EAX],XMM3
                    self.xmm[3] = load_vec(self.e, self.ecx); // MOVAPS XMM3,xmmword ptr [ECX]
                    self.xmm[3] = sub(self.xmm[3], self.xmm[4]); // SUBPS XMM3,XMM4
                    store_vec(self.e, self.ecx, self.xmm[3]); // MOVAPS xmmword ptr [ECX],XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_sub(0x20)); // MOVAPS XMM3,xmmword ptr [ESI + -0x20]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[2]); // MULPS XMM3,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_sub(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + -0x10]
                    self.edx = self.ebx.wrapping_add(0x20); // LEA EDX,[EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[1]); // MULPS XMM2,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM1,dword ptr [EDI]
                    self.xmm[3] = add(self.xmm[3], self.xmm[4]); // ADDPS XMM3,XMM4
                    store_vec(self.e, self.edx, self.xmm[3]); // MOVAPS xmmword ptr [EDX],XMM3
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EDX,dword ptr [ESP + 0x30]
                    self.xmm[3] = load_vec(self.e, self.edx); // MOVAPS XMM3,xmmword ptr [EDX]
                    self.xmm[3] = add(self.xmm[3], self.xmm[2]); // ADDPS XMM3,XMM2
                    store_vec(self.e, self.edx, self.xmm[3]); // MOVAPS xmmword ptr [EDX],XMM3
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.e.mem.set_u32(self.edi, self.xmm[1][0].to_bits()); // MOVSS dword ptr [EDI],XMM1
                    self.edx = i32::from(self.e.mem.i8(self.esi.wrapping_sub(0x30))) as u32; // MOVSX EDX,byte ptr [ESI + -0x30]
                    self.edx = u32::from(self.e.mem.u8(self.edx.wrapping_add(0x11b9434))); // MOVZX EDX,byte ptr [EDX + 0x11b9434]
                    self.edi = self.edi.wrapping_add(self.edx.wrapping_mul(4)); // LEA EDI,[EDI + EDX*0x4]
                    self.edx = (self.edx & 0xffffff00) | u32::from(self.e.mem.u8(self.esi)); // MOV DL,byte ptr [ESI]
                    self.cmp8(self.edx as u8, 0x10); // CMP DL,0x10
                    if self.zf {
                        // JZ 0x00d90200
                        block = 0x00d9_0200;
                        continue;
                    }
                    block = 0x00d9_02ff;
                }
                0x00d9_02ff => {
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_0304 => {
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EDX,dword ptr [ESP + 0x4c]
                    self.eax = self.ecx.wrapping_add(0x30); // LEA EAX,[ECX + 0x30]
                    self.ecx = self.add32(self.ecx, 0x10); // ADD ECX,0x10
                    self.edx = self.add32(self.edx, 0x20); // ADD EDX,0x20
                    self.e.mem.set_u32(self.esp.wrapping_add(0x50), self.eax); // MOV dword ptr [ESP + 0x50],EAX
                    self.eax = self.ebx.wrapping_add(0x10); // LEA EAX,[EBX + 0x10]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x10), self.ecx); // MOV dword ptr [ESP + 0x10],ECX
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.edx); // MOV dword ptr [ESP + 0x30],EDX
                    block = 0x00d9_0324;
                }
                0x00d9_0324 => {
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x10)); // MOV EDX,dword ptr [ESP + 0x10]
                    self.xmm[0] = load_vec(self.e, self.edx); // MOVAPS XMM0,xmmword ptr [EDX]
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EDX,dword ptr [ESP + 0x30]
                    self.xmm[3] = load_vec(self.e, self.edx); // MOVAPS XMM3,xmmword ptr [EDX]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi); // MOVAPS XMM3,xmmword ptr [ESI]
                    self.xmm[4] = mul(self.xmm[4], self.xmm[2]); // MULPS XMM4,XMM2
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x50)); // MOV EDX,dword ptr [ESP + 0x50]
                    self.xmm[0] = add(self.xmm[0], self.xmm[4]); // ADDPS XMM0,XMM4
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM1,dword ptr [EDI]
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[3], 0xff); // SHUFPS XMM3,XMM3,0xff
                    self.xmm[3] = sub_ss(self.xmm[3], self.xmm[0]); // SUBSS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[3] = mul_ss(self.xmm[3], self.xmm[2]); // MULSS XMM3,XMM2
                    self.xmm[2] = load_vec(self.e, VECTOR_CONSTANT_010C7160); // MOVAPS XMM2,xmmword ptr [0x010c7160]
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[1]); // SUBSS XMM2,XMM1
                    self.xmm[2] = max_ps(self.xmm[2], self.xmm[3]); // MAXPS XMM2,XMM3
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xff); // SHUFPS XMM0,XMM0,0xff
                    self.xmm[0] = sub_ss(self.xmm[0], self.xmm[1]); // SUBSS XMM0,XMM1
                    self.xmm[0] = min_ps(self.xmm[0], self.xmm[2]); // MINPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[4], 0x0); // SHUFPS XMM0,XMM4,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.edx); // MOVAPS XMM2,xmmword ptr [EDX]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[1], 0xff); // SHUFPS XMM2,XMM1,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[0], 0xff); // SHUFPS XMM3,XMM0,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.eax); // MOVAPS XMM5,xmmword ptr [EAX]
                    self.xmm[2] = add(self.xmm[2], self.xmm[5]); // ADDPS XMM2,XMM5
                    store_vec(self.e, self.eax, self.xmm[2]); // MOVAPS xmmword ptr [EAX],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx); // MOVAPS XMM2,xmmword ptr [ECX]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx, self.xmm[2]); // MOVAPS xmmword ptr [ECX],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[1]); // MULPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [ESI + 0x20]
                    self.edx = self.ebx.wrapping_add(0x20); // LEA EDX,[EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM1,dword ptr [EDI]
                    self.xmm[2] = add(self.xmm[2], self.xmm[3]); // ADDPS XMM2,XMM3
                    store_vec(self.e, self.edx, self.xmm[2]); // MOVAPS xmmword ptr [EDX],XMM2
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EDX,dword ptr [ESP + 0x30]
                    self.xmm[2] = load_vec(self.e, self.edx); // MOVAPS XMM2,xmmword ptr [EDX]
                    self.xmm[0] = add(self.xmm[0], self.xmm[2]); // ADDPS XMM0,XMM2
                    store_vec(self.e, self.edx, self.xmm[0]); // MOVAPS xmmword ptr [EDX],XMM0
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[4]); // ADDSS XMM1,XMM4
                    self.e.mem.set_u32(self.edi, self.xmm[1][0].to_bits()); // MOVSS dword ptr [EDI],XMM1
                    self.edx = i32::from(self.e.mem.i8(self.esi)) as u32; // MOVSX EDX,byte ptr [ESI]
                    self.edx = u32::from(self.e.mem.u8(self.edx.wrapping_add(0x11b9434))); // MOVZX EDX,byte ptr [EDX + 0x11b9434]
                    self.edi = self.edi.wrapping_add(self.edx.wrapping_mul(4)); // LEA EDI,[EDI + EDX*0x4]
                    self.edx = (self.edx & 0xffffff00)
                        | u32::from(self.e.mem.u8(self.esi.wrapping_add(0x30))); // MOV DL,byte ptr [ESI + 0x30]
                    self.esi = self.add32(self.esi, 0x30); // ADD ESI,0x30
                    self.cmp8(self.edx as u8, 0x12); // CMP DL,0x12
                    if self.zf {
                        // JZ 0x00d90324
                        block = 0x00d9_0324;
                        continue;
                    }
                    block = 0x00d9_042f;
                }
                0x00d9_042f => {
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_0434 => {
                    self.fmul32(self.esi.wrapping_add(0x4c)); // FMUL float ptr [ESI + 0x4c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fld32(self.edi.wrapping_add(0x4)); // FLD float ptr [EDI + 0x4]
                    self.x87.fchs(); // FCHS
                    self.fcompp(); // FCOMPP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x1); // TEST AH,0x1
                    if !self.zf {
                        // JNZ 0x00d904f4
                        block = 0x00d9_04f4;
                        continue;
                    }
                    block = 0x00d9_0451;
                }
                0x00d9_0451 => {
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.xmm[1] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x1a0))); // MOVSS XMM1,dword ptr [ESP + 0x1a0]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EBX + 0x30]
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x50)); // MOV EAX,dword ptr [ESP + 0x50]
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xff); // SHUFPS XMM0,XMM0,0xff
                    self.xmm[1] = mul_ss(self.xmm[1], self.xmm[0]); // MULSS XMM1,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM0,dword ptr [EDI]
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, VECTOR_CONSTANT_010C7160); // MOVAPS XMM0,xmmword ptr [0x010c7160]
                    self.xmm[0] = sub_ss(self.xmm[0], self.xmm[2]); // SUBSS XMM0,XMM2
                    self.xmm[0] = max_ps(self.xmm[0], self.xmm[1]); // MAXPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x0); // SHUFPS XMM1,XMM0,0x0
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[2] = mul(self.xmm[2], self.xmm[3]); // MULPS XMM2,XMM3
                    self.xmm[3] = load_vec(self.e, self.eax); // MOVAPS XMM3,xmmword ptr [EAX]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = self.xmm[2]; // MOVAPS XMM3,XMM2
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[2], 0xff); // SHUFPS XMM3,XMM2,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[4] = self.xmm[1]; // MOVAPS XMM4,XMM1
                    self.xmm[4] = shufps(self.xmm[4], self.xmm[1], 0xff); // SHUFPS XMM4,XMM1,0xff
                    self.xmm[4] = mul(self.xmm[4], self.xmm[5]); // MULPS XMM4,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = add(self.xmm[3], self.xmm[5]); // ADDPS XMM3,XMM5
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[3]); // MOVAPS xmmword ptr [EBX + 0x10],XMM3
                    self.xmm[3] = load_vec(self.e, self.ecx); // MOVAPS XMM3,xmmword ptr [ECX]
                    self.xmm[3] = sub(self.xmm[3], self.xmm[4]); // SUBPS XMM3,XMM4
                    store_vec(self.e, self.ecx, self.xmm[3]); // MOVAPS xmmword ptr [ECX],XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[2]); // MULPS XMM3,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ESI + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[1]); // MULPS XMM2,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM1,dword ptr [EDI]
                    self.xmm[3] = add(self.xmm[3], self.xmm[4]); // ADDPS XMM3,XMM4
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[3]); // MOVAPS xmmword ptr [EBX + 0x20],XMM3
                    self.xmm[3] = load_vec(self.e, self.edx); // MOVAPS XMM3,xmmword ptr [EDX]
                    self.xmm[3] = add(self.xmm[3], self.xmm[2]); // ADDPS XMM3,XMM2
                    store_vec(self.e, self.edx, self.xmm[3]); // MOVAPS xmmword ptr [EDX],XMM3
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.e.mem.set_u32(self.edi, self.xmm[1][0].to_bits()); // MOVSS dword ptr [EDI],XMM1
                    block = 0x00d9_0584; // JMP 0x00d90584
                }
                0x00d9_04f4 => {
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.eax = self.edi.wrapping_add(0x4); // LEA EAX,[EDI + 0x4]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.eax); // MOV dword ptr [ESP + 0x30],EAX
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.eax = self.esi.wrapping_add(0x30); // LEA EAX,[ESI + 0x30]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x10), self.eax); // MOV dword ptr [ESP + 0x10],EAX
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x50)); // MOV EAX,dword ptr [ESP + 0x50]
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[4], 0x0); // SHUFPS XMM0,XMM4,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax); // MOVAPS XMM2,xmmword ptr [EAX]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x10)); // MOV EAX,dword ptr [ESP + 0x10]
                    self.xmm[5] = load_vec(self.e, self.eax); // MOVAPS XMM5,xmmword ptr [EAX]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[1], 0xff); // SHUFPS XMM2,XMM1,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[0], 0xff); // SHUFPS XMM3,XMM0,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[5] = add(self.xmm[5], self.xmm[2]); // ADDPS XMM5,XMM2
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[5]); // MOVAPS xmmword ptr [EBX + 0x10],XMM5
                    self.xmm[2] = load_vec(self.e, self.ecx); // MOVAPS XMM2,xmmword ptr [ECX]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx, self.xmm[2]); // MOVAPS xmmword ptr [ECX],XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[1]); // MULPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EAX,dword ptr [ESP + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.eax)); // MOVSS XMM0,dword ptr [EAX]
                    self.xmm[2] = add(self.xmm[2], self.xmm[3]); // ADDPS XMM2,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x20],XMM2
                    block = 0x00d9_0573;
                }
                0x00d9_0573 => {
                    self.xmm[2] = load_vec(self.e, self.edx); // MOVAPS XMM2,xmmword ptr [EDX]
                    self.xmm[2] = add(self.xmm[2], self.xmm[1]); // ADDPS XMM2,XMM1
                    store_vec(self.e, self.edx, self.xmm[2]); // MOVAPS xmmword ptr [EDX],XMM2
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[4]); // ADDSS XMM0,XMM4
                    self.e.mem.set_u32(self.eax, self.xmm[0][0].to_bits()); // MOVSS dword ptr [EAX],XMM0
                    block = 0x00d9_0584;
                }
                0x00d9_0584 => {
                    self.esi = self.add32(self.esi, 0x70); // ADD ESI,0x70
                    self.edi = self.add32(self.edi, 0x8); // ADD EDI,0x8
                    self.cmp8(self.e.mem.u8(self.esi), 0x17); // CMP byte ptr [ESI],0x17
                    if !self.zf {
                        // JNZ 0x00d8fbfc
                        block = 0x00d8_fbfc;
                        continue;
                    }
                    block = 0x00d9_0593;
                }
                0x00d9_0593 => {
                    block = 0x00d9_076a; // JMP 0x00d9076a
                }
                0x00d9_0598 => {
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EBX + 0x20]
                    self.fld32(self.esi.wrapping_add(0x40)); // FLD float ptr [ESI + 0x40]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [EAX + 0x20]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.xmm[0] = load_vec(self.e, self.esi); // MOVAPS XMM0,xmmword ptr [ESI]
                    self.x87.fmul_sti(0); // FMUL ST0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.fld32(self.esi.wrapping_add(0xc)); // FLD float ptr [ESI + 0xc]
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [ESI + 0x10]
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0xff); // SHUFPS XMM1,XMM1,0xff
                    self.xmm[1] = sub_ss(self.xmm[1], self.xmm[0]); // SUBSS XMM1,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xac), self.xmm[1][0].to_bits()); // MOVSS dword ptr [ESP + 0xac],XMM1
                    self.fmul32(self.esp.wrapping_add(0xac)); // FMUL float ptr [ESP + 0xac]
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [ESI + 0x20]
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [EAX + 0x20]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [ESI + 0x30]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [ESI + 0x30]
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0xff); // SHUFPS XMM1,XMM1,0xff
                    self.xmm[1] = sub_ss(self.xmm[1], self.xmm[0]); // SUBSS XMM1,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x8c), self.xmm[1][0].to_bits()); // MOVSS dword ptr [ESP + 0x8c],XMM1
                    self.fld32(self.esi.wrapping_add(0x2c)); // FLD float ptr [ESI + 0x2c]
                    self.fmul32(self.esp.wrapping_add(0x8c)); // FMUL float ptr [ESP + 0x8c]
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.fld32(self.esp.wrapping_add(0x30)); // FLD float ptr [ESP + 0x30]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.x87.fmul_sti(0); // FMUL ST0
                    self.x87.fld_st(1); // FLD ST1
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x50)); // FSTP float ptr [ESP + 0x50]
                    self.fld32(self.esp.wrapping_add(0x50)); // FLD float ptr [ESP + 0x50]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fcom_st(1, false); // FCOM
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d906a4
                        block = 0x00d9_06a4;
                        continue;
                    }
                    block = 0x00d9_066f;
                }
                0x00d9_066f => {
                    self.x87.fdivrp(1); // FDIVRP
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.call_square_root(); // CALL 0x00ec6040
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x30)); // FMUL float ptr [ESP + 0x30]
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    block = 0x00d9_06aa; // JMP 0x00d906aa
                }
                0x00d9_06a4 => {
                    self.x87.fstp_st(1); // FSTP ST1
                    self.x87.pop(); // FSTP ST0
                    self.x87.push(1.0); // FLD1
                    block = 0x00d9_06aa;
                }
                0x00d9_06aa => {
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.fstp32(self.edi.wrapping_add(0x8)); // FSTP float ptr [EDI + 0x8]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EBX + 0x30]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[2], 0x0); // SHUFPS XMM0,XMM2,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EAX + 0x30]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi); // MOVAPS XMM3,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM0,dword ptr [EDI]
                    self.xmm[4] = add(self.xmm[4], self.xmm[1]); // ADDPS XMM4,XMM1
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[4]); // MOVAPS xmmword ptr [EBX + 0x20],XMM4
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    self.xmm[1] = add(self.xmm[1], self.xmm[3]); // ADDPS XMM1,XMM3
                    store_vec(self.e, self.eax.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [EAX + 0x20],XMM1
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[2]); // ADDSS XMM0,XMM2
                    self.e.mem.set_u32(self.edi, self.xmm[0][0].to_bits()); // MOVSS dword ptr [EDI],XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x30))); // MOVSS XMM0,dword ptr [ESP + 0x30]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EBX + 0x30]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[2], 0x0); // SHUFPS XMM0,XMM2,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EAX + 0x30]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [ESI + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [ESI + 0x30]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.edi.wrapping_add(0x4))); // MOVSS XMM0,dword ptr [EDI + 0x4]
                    self.xmm[1] = add(self.xmm[1], self.xmm[4]); // ADDPS XMM1,XMM4
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [EBX + 0x20],XMM1
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    self.xmm[1] = add(self.xmm[1], self.xmm[3]); // ADDPS XMM1,XMM3
                    store_vec(self.e, self.eax.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [EAX + 0x20],XMM1
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[2]); // ADDSS XMM0,XMM2
                    self.push(self.esi); // PUSH ESI
                    self.e
                        .mem
                        .set_u32(self.edi.wrapping_add(0x4), self.xmm[0][0].to_bits()); // MOVSS dword ptr [EDI + 0x4],XMM0
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.eax = i32::from(self.e.mem.i8(self.esi.wrapping_add(0x50))) as u32; // MOVSX EAX,byte ptr [ESI + 0x50]
                    self.esp = self.add32(self.esp, 0x4); // ADD ESP,0x4
                    self.esi = self.add32(self.esi, 0x50); // ADD ESI,0x50
                    self.cmp32(self.eax, 0x17); // CMP EAX,0x17
                    if !self.zf {
                        // JNZ 0x00d90ab3
                        block = 0x00d9_0ab3;
                        continue;
                    }
                    block = 0x00d9_076a;
                }
                0x00d9_076a => {
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.xmm[4] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [ECX + 0x20]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[2] = load_vec(self.e, self.esi); // MOVAPS XMM2,xmmword ptr [ESI]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[4]); // MULPS XMM0,XMM4
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x94), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0x94],XMM2
                    self.fld32(self.esp.wrapping_add(0x94)); // FLD float ptr [ESP + 0x94]
                    self.fst32(self.esp.wrapping_add(0x1b0)); // FST float ptr [ESP + 0x1b0]
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x40)); // MOVAPS XMM3,xmmword ptr [ESI + 0x40]
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ECX + 0x20]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x50)); // MOVAPS XMM0,xmmword ptr [ESI + 0x50]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [ESI + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xb4), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0xb4],XMM2
                    self.fld32(self.esp.wrapping_add(0xb4)); // FLD float ptr [ESP + 0xb4]
                    self.fst32(self.esp.wrapping_add(0x1b4)); // FST float ptr [ESP + 0x1b4]
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x60)); // MOVAPS XMM1,xmmword ptr [ESI + 0x60]
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ECX + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x70)); // MOVAPS XMM0,xmmword ptr [ESI + 0x70]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x70)); // MOVAPS XMM1,xmmword ptr [ESI + 0x70]
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0xff); // SHUFPS XMM1,XMM1,0xff
                    self.xmm[1] = sub_ss(self.xmm[1], self.xmm[0]); // SUBSS XMM1,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x9c), self.xmm[1][0].to_bits()); // MOVSS dword ptr [ESP + 0x9c],XMM1
                    self.fld32(self.esp.wrapping_add(0x9c)); // FLD float ptr [ESP + 0x9c]
                    self.fst32(self.esp.wrapping_add(0x1b8)); // FST float ptr [ESP + 0x1b8]
                    self.fmul32(self.esi.wrapping_add(0x6c)); // FMUL float ptr [ESI + 0x6c]
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.fld32(self.esi.wrapping_add(0x2c)); // FLD float ptr [ESI + 0x2c]
                    self.x87.fmul_st0(2); // FMUL ST2
                    self.fld32(self.esi.wrapping_add(0x80)); // FLD float ptr [ESI + 0x80]
                    self.x87.fmul_st0(2); // FMUL ST2
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x50)); // FSTP float ptr [ESP + 0x50]
                    self.fmul32(self.esi.wrapping_add(0x5c)); // FMUL float ptr [ESI + 0x5c]
                    self.fld32(self.esi.wrapping_add(0x80)); // FLD float ptr [ESI + 0x80]
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x50)); // FLD float ptr [ESP + 0x50]
                    self.fld32(self.esp.wrapping_add(0x30)); // FLD float ptr [ESP + 0x30]
                    self.x87.fld_st(1); // FLD ST1
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.fld_st(2); // FLD ST2
                    self.x87.fmulp(3); // FMULP ST3
                    self.x87.fxch(1); // FXCH
                    self.x87.faddp(2); // FADDP ST2,ST0
                    self.x87.fmul_sti(0); // FMUL ST0
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esi.wrapping_add(0x84)); // FLD float ptr [ESI + 0x84]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.x87.fld_st(1); // FLD ST1
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.fxch(1); // FXCH
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fcom_st(1, true); // FCOMP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d90933
                        block = 0x00d9_0933;
                        continue;
                    }
                    block = 0x00d9_08e8;
                }
                0x00d9_08e8 => {
                    self.call_square_root(); // CALL 0x00ec6040
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.x87.push(1.0); // FLD1
                    self.x87.fdivrp(1); // FDIVRP
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fmul32(self.esi.wrapping_add(0x84)); // FMUL float ptr [ESI + 0x84]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x50)); // FMUL float ptr [ESP + 0x50]
                    self.fstp32(self.esp.wrapping_add(0x50)); // FSTP float ptr [ESP + 0x50]
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x30)); // FMUL float ptr [ESP + 0x30]
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    block = 0x00d9_0937; // JMP 0x00d90937
                }
                0x00d9_0933 => {
                    self.x87.pop(); // FSTP ST0
                    self.x87.push(1.0); // FLD1
                    block = 0x00d9_0937;
                }
                0x00d9_0937 => {
                    self.fstp32(self.edi.wrapping_add(0xc)); // FSTP float ptr [EDI + 0xc]
                    self.xmm[5] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x50))); // MOVSS XMM5,dword ptr [ESP + 0x50]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.fld32(self.esi.wrapping_add(0x88)); // FLD float ptr [ESI + 0x88]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.fmul32(self.esp.wrapping_add(0x30)); // FMUL float ptr [ESP + 0x30]
                    self.xmm[6] = load_vec(self.e, self.esi); // MOVAPS XMM6,xmmword ptr [ESI]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[1] = self.xmm[5]; // MOVAPS XMM1,XMM5
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[5], 0x0); // SHUFPS XMM1,XMM5,0x0
                    self.xmm[0] = self.xmm[1]; // MOVAPS XMM0,XMM1
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [ECX + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[6]); // MULPS XMM2,XMM6
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[6]); // MULPS XMM3,XMM6
                    self.xmm[6] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM6,xmmword ptr [EBX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[6]); // ADDPS XMM2,XMM6
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ECX + 0x10]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [ECX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ESI + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x20],XMM0
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ECX + 0x20]
                    self.xmm[1] = add(self.xmm[1], self.xmm[0]); // ADDPS XMM1,XMM0
                    store_vec(self.e, self.ecx.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [ECX + 0x20],XMM1
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[5]); // ADDSS XMM2,XMM5
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.xmm[5] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM5,xmmword ptr [ESI + 0x30]
                    self.xmm[0] = self.xmm[4]; // MOVAPS XMM0,XMM4
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[4], 0x0); // SHUFPS XMM0,XMM4,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [ECX + 0x30]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[1], 0xff); // SHUFPS XMM2,XMM1,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[0], 0xff); // SHUFPS XMM3,XMM0,0xff
                    self.eax = self.esi.wrapping_add(0x30); // LEA EAX,[ESI + 0x30]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[5]); // ADDPS XMM2,XMM5
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ECX + 0x10]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [ECX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[1]); // MULPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.edi.wrapping_add(0x4))); // MOVSS XMM0,dword ptr [EDI + 0x4]
                    self.xmm[2] = add(self.xmm[2], self.xmm[3]); // ADDPS XMM2,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x20],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ECX + 0x20]
                    self.xmm[1] = add(self.xmm[1], self.xmm[2]); // ADDPS XMM1,XMM2
                    store_vec(self.e, self.ecx.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [ECX + 0x20],XMM1
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[4]); // ADDSS XMM0,XMM4
                    self.e
                        .mem
                        .set_u32(self.edi.wrapping_add(0x4), self.xmm[0][0].to_bits()); // MOVSS dword ptr [EDI + 0x4],XMM0
                    self.xmm[3] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x30))); // MOVSS XMM3,dword ptr [ESP + 0x30]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.xmm[0] = self.xmm[3]; // MOVAPS XMM0,XMM3
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[3], 0x0); // SHUFPS XMM0,XMM3,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [ECX + 0x30]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x60)); // MOVAPS XMM2,xmmword ptr [ESI + 0x60]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x70)); // MOVAPS XMM2,xmmword ptr [ESI + 0x70]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.edi.wrapping_add(0x8))); // MOVSS XMM0,dword ptr [EDI + 0x8]
                    self.xmm[1] = add(self.xmm[1], self.xmm[4]); // ADDPS XMM1,XMM4
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [EBX + 0x20],XMM1
                    self.xmm[1] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [ECX + 0x20]
                    self.xmm[2] = add(self.xmm[2], self.xmm[1]); // ADDPS XMM2,XMM1
                    store_vec(self.e, self.ecx.wrapping_add(0x20), self.xmm[2]); // MOVAPS xmmword ptr [ECX + 0x20],XMM2
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[3]); // ADDSS XMM0,XMM3
                    self.e
                        .mem
                        .set_u32(self.edi.wrapping_add(0x8), self.xmm[0][0].to_bits()); // MOVSS dword ptr [EDI + 0x8],XMM0
                    self.eax = (self.eax & 0xffffff00)
                        | u32::from(self.e.mem.u8(self.esi.wrapping_add(0x90))); // MOV AL,byte ptr [ESI + 0x90]
                    self.edi = self.add32(self.edi, 0x10); // ADD EDI,0x10
                    self.esi = self.add32(self.esi, 0x90); // ADD ESI,0x90
                    self.cmp8(self.eax as u8, 0x1); // CMP AL,0x1
                    if !self.zf {
                        // JNZ 0x00d8fbfc
                        block = 0x00d8_fbfc;
                        continue;
                    }
                    block = 0x00d9_0aae;
                }
                0x00d9_0aae => {
                    block = 0x00d8_fbe0; // JMP 0x00d8fbe0
                }
                0x00d9_0ab3 => {
                    self.cmp32(self.eax, 0x16); // CMP EAX,0x16
                    if !self.zf {
                        // JNZ 0x00d8fbfc
                        block = 0x00d8_fbfc;
                        continue;
                    }
                    block = 0x00d9_0abc;
                }
                0x00d9_0abc => {
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.xmm[4] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [ECX + 0x20]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[2] = load_vec(self.e, self.esi); // MOVAPS XMM2,xmmword ptr [ESI]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[4]); // MULPS XMM0,XMM4
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xc4), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0xc4],XMM2
                    self.fld32(self.esp.wrapping_add(0xc4)); // FLD float ptr [ESP + 0xc4]
                    self.fst32(self.esp.wrapping_add(0x250)); // FST float ptr [ESP + 0x250]
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x40)); // MOVAPS XMM3,xmmword ptr [ESI + 0x40]
                    self.xmm[1] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [ECX + 0x20]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[0]); // SUBPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x50)); // MOVAPS XMM0,xmmword ptr [ESI + 0x50]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [ESI + 0x30]
                    self.xmm[4] = self.xmm[1]; // MOVAPS XMM4,XMM1
                    self.xmm[4] = mul(self.xmm[4], self.xmm[2]); // MULPS XMM4,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[4]); // ADDPS XMM0,XMM4
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0x55); // SHUFPS XMM2,XMM0,0x55
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[0]); // ADDSS XMM2,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[2]); // ADDSS XMM0,XMM2
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0xff); // SHUFPS XMM1,XMM1,0xff
                    self.xmm[1] = sub_ss(self.xmm[1], self.xmm[0]); // SUBSS XMM1,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xa4), self.xmm[1][0].to_bits()); // MOVSS dword ptr [ESP + 0xa4],XMM1
                    self.fld32(self.esp.wrapping_add(0xa4)); // FLD float ptr [ESP + 0xa4]
                    self.fst32(self.esp.wrapping_add(0x254)); // FST float ptr [ESP + 0x254]
                    self.fld32(self.esi.wrapping_add(0x64)); // FLD float ptr [ESI + 0x64]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.x87.fmul_sti(0); // FMUL ST0
                    self.fstp32(self.esp.wrapping_add(0x50)); // FSTP float ptr [ESP + 0x50]
                    self.fld32(self.esi.wrapping_add(0x2c)); // FLD float ptr [ESI + 0x2c]
                    self.x87.fmul_st0(2); // FMUL ST2
                    self.x87.fld_st(1); // FLD ST1
                    self.fmul32(self.esi.wrapping_add(0x60)); // FMUL float ptr [ESI + 0x60]
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.fmul32(self.esi.wrapping_add(0x5c)); // FMUL float ptr [ESI + 0x5c]
                    self.fld32(self.esi.wrapping_add(0x60)); // FLD float ptr [ESI + 0x60]
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x30)); // FLD float ptr [ESP + 0x30]
                    self.x87.fmul_sti(0); // FMUL ST0
                    self.x87.fld_st(1); // FLD ST1
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.faddp(1); // FADDP
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x50)); // FLD float ptr [ESP + 0x50]
                    self.fcom_st(1, false); // FCOM
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d90c0f
                        block = 0x00d9_0c0f;
                        continue;
                    }
                    block = 0x00d9_0bd6;
                }
                0x00d9_0bd6 => {
                    self.x87.fdivrp(1); // FDIVRP
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.call_square_root(); // CALL 0x00ec6040
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x30)); // FMUL float ptr [ESP + 0x30]
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.x87.fld_st(0); // FLD ST0
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    block = 0x00d9_0c15; // JMP 0x00d90c15
                }
                0x00d9_0c0f => {
                    self.x87.fstp_st(1); // FSTP ST1
                    self.x87.pop(); // FSTP ST0
                    self.x87.push(1.0); // FLD1
                    block = 0x00d9_0c15;
                }
                0x00d9_0c15 => {
                    self.fstp32(self.edi.wrapping_add(0x8)); // FSTP float ptr [EDI + 0x8]
                    self.xmm[5] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x30))); // MOVSS XMM5,dword ptr [ESP + 0x30]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[1] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [ECX + 0x30]
                    self.xmm[6] = load_vec(self.e, self.esi); // MOVAPS XMM6,xmmword ptr [ESI]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[2] = self.xmm[5]; // MOVAPS XMM2,XMM5
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[5], 0x0); // SHUFPS XMM2,XMM5,0x0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[6]); // MULPS XMM2,XMM6
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[6]); // MULPS XMM3,XMM6
                    self.xmm[6] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM6,xmmword ptr [EBX + 0x10]
                    self.xmm[6] = add(self.xmm[6], self.xmm[2]); // ADDPS XMM6,XMM2
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[6]); // MOVAPS xmmword ptr [EBX + 0x10],XMM6
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ECX + 0x10]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [ECX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[3] = add(self.xmm[3], self.xmm[2]); // ADDPS XMM3,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM1,dword ptr [EDI]
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[3]); // MOVAPS xmmword ptr [EBX + 0x20],XMM3
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ECX + 0x20]
                    self.xmm[2] = add(self.xmm[2], self.xmm[0]); // ADDPS XMM2,XMM0
                    store_vec(self.e, self.ecx.wrapping_add(0x20), self.xmm[2]); // MOVAPS xmmword ptr [ECX + 0x20],XMM2
                    self.eax = self.esi.wrapping_add(0x30); // LEA EAX,[ESI + 0x30]
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[5]); // ADDSS XMM1,XMM5
                    self.e.mem.set_u32(self.edi, self.xmm[1][0].to_bits()); // MOVSS dword ptr [EDI],XMM1
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[3] = load_vec(self.e, self.eax); // MOVAPS XMM3,xmmword ptr [EAX]
                    self.xmm[1] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [ECX + 0x30]
                    self.xmm[2] = self.xmm[4]; // MOVAPS XMM2,XMM4
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[4], 0x0); // SHUFPS XMM2,XMM4,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[5] = self.xmm[0]; // MOVAPS XMM5,XMM0
                    self.xmm[5] = shufps(self.xmm[5], self.xmm[0], 0xff); // SHUFPS XMM5,XMM0,0xff
                    self.xmm[2] = self.xmm[3]; // MOVAPS XMM2,XMM3
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[5]); // ADDPS XMM2,XMM5
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ECX + 0x10]
                    self.xmm[6] = self.xmm[1]; // MOVAPS XMM6,XMM1
                    self.xmm[6] = shufps(self.xmm[6], self.xmm[1], 0xff); // SHUFPS XMM6,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[6]); // MULPS XMM3,XMM6
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [ECX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EAX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.edi.wrapping_add(0x4))); // MOVSS XMM1,dword ptr [EDI + 0x4]
                    self.xmm[2] = add(self.xmm[2], self.xmm[3]); // ADDPS XMM2,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x20],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ECX + 0x20]
                    self.xmm[2] = add(self.xmm[2], self.xmm[0]); // ADDPS XMM2,XMM0
                    store_vec(self.e, self.ecx.wrapping_add(0x20), self.xmm[2]); // MOVAPS xmmword ptr [ECX + 0x20],XMM2
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[4]); // ADDSS XMM1,XMM4
                    self.e
                        .mem
                        .set_u32(self.edi.wrapping_add(0x4), self.xmm[1][0].to_bits()); // MOVSS dword ptr [EDI + 0x4],XMM1
                    self.eax = (self.eax & 0xffffff00)
                        | u32::from(self.e.mem.u8(self.esi.wrapping_add(0x70))); // MOV AL,byte ptr [ESI + 0x70]
                    self.edi = self.add32(self.edi, 0xc); // ADD EDI,0xc
                    self.esi = self.add32(self.esi, 0x70); // ADD ESI,0x70
                    self.cmp8(self.eax as u8, 0x1); // CMP AL,0x1
                    if !self.zf {
                        // JNZ 0x00d8fbfc
                        block = 0x00d8_fbfc;
                        continue;
                    }
                    block = 0x00d9_0d24;
                }
                0x00d9_0d24 => {
                    block = 0x00d8_fbe0; // JMP 0x00d8fbe0
                }
                0x00d9_0d29 => {
                    self.xmm[0] = load_vec(self.e, self.esi); // MOVAPS XMM0,xmmword ptr [ESI]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [EBX + 0x20]
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [ESI + 0x10]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.xmm[3] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [ECX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = self.xmm[1]; // MOVAPS XMM0,XMM1
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[2]); // ADDPS XMM0,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0x55); // SHUFPS XMM2,XMM0,0x55
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[0]); // ADDSS XMM2,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[2]); // ADDSS XMM0,XMM2
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0xff); // SHUFPS XMM1,XMM1,0xff
                    self.xmm[1] = sub_ss(self.xmm[1], self.xmm[0]); // SUBSS XMM1,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xbc), self.xmm[1][0].to_bits()); // MOVSS dword ptr [ESP + 0xbc],XMM1
                    self.fld32(self.esi.wrapping_add(0xc)); // FLD float ptr [ESI + 0xc]
                    self.fmul32(self.esp.wrapping_add(0xbc)); // FMUL float ptr [ESP + 0xbc]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.x87.fld_st(0); // FLD ST0
                    self.x87.fabs(); // FABS
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fld32(self.esi.wrapping_add(0x20)); // FLD float ptr [ESI + 0x20]
                    self.fcom_st(1, true); // FCOMP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d90db5
                        block = 0x00d9_0db5;
                        continue;
                    }
                    block = 0x00d9_0d9e;
                }
                0x00d9_0d9e => {
                    self.fdivr32(self.esi.wrapping_add(0x20)); // FDIVR float ptr [ESI + 0x20]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.x87.fld_st(0); // FLD ST0
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.fxch(1); // FXCH
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    block = 0x00d9_0dbb; // JMP 0x00d90dbb
                }
                0x00d9_0db5 => {
                    self.x87.pop(); // FSTP ST0
                    self.x87.pop(); // FSTP ST0
                    self.x87.push(1.0); // FLD1
                    block = 0x00d9_0dbb;
                }
                0x00d9_0dbb => {
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.fstp32(self.edi.wrapping_add(0x4)); // FSTP float ptr [EDI + 0x4]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EBX + 0x30]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[2], 0x0); // SHUFPS XMM0,XMM2,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EAX + 0x30]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi); // MOVAPS XMM3,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM0,dword ptr [EDI]
                    self.xmm[4] = add(self.xmm[4], self.xmm[1]); // ADDPS XMM4,XMM1
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[4]); // MOVAPS xmmword ptr [EBX + 0x20],XMM4
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    self.xmm[1] = add(self.xmm[1], self.xmm[3]); // ADDPS XMM1,XMM3
                    store_vec(self.e, self.eax.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [EAX + 0x20],XMM1
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[2]); // ADDSS XMM0,XMM2
                    self.push(self.esi); // PUSH ESI
                    self.e.mem.set_u32(self.edi, self.xmm[0][0].to_bits()); // MOVSS dword ptr [EDI],XMM0
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.eax = (self.eax & 0xffffff00)
                        | u32::from(self.e.mem.u8(self.esi.wrapping_add(0x30))); // MOV AL,byte ptr [ESI + 0x30]
                    self.esp = self.add32(self.esp, 0x4); // ADD ESP,0x4
                    self.esi = self.add32(self.esi, 0x30); // ADD ESI,0x30
                    self.cmp8(self.eax as u8, 0xe); // CMP AL,0xe
                    if self.zf {
                        // JZ 0x00d90d29
                        block = 0x00d9_0d29;
                        continue;
                    }
                    block = 0x00d9_0e29;
                }
                0x00d9_0e29 => {
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_0e30 => {
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x50)); // MOVAPS XMM1,xmmword ptr [EBX + 0x50]
                    self.fld32(self.esi.wrapping_add(0x28)); // FLD float ptr [ESI + 0x28]
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.fld32(self.esi.wrapping_add(0x1c)); // FLD float ptr [ESI + 0x1c]
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ECX + 0x20]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x50)); // MOV EDX,dword ptr [EBX + 0x50]
                    self.eax = self.e.mem.u32(self.ebp.wrapping_add(0x8)); // MOV EAX,dword ptr [EBP + 0x8]
                    self.xmm[3] = movss_load(self.e.mem.u32(self.eax.wrapping_add(0x8))); // MOVSS XMM3,dword ptr [EAX + 0x8]
                    self.xmm[0] = sub(self.xmm[0], self.xmm[1]); // SUBPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.ecx.wrapping_add(0x50)); // MOVAPS XMM1,xmmword ptr [ECX + 0x50]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[1]); // SUBPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi); // MOVAPS XMM1,xmmword ptr [ESI]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [ESI + 0x10]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x180), self.edx); // MOV dword ptr [ESP + 0x180],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x54)); // MOV EDX,dword ptr [EBX + 0x54]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x184), self.edx); // MOV dword ptr [ESP + 0x184],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x58)); // MOV EDX,dword ptr [EBX + 0x58]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x188), self.edx); // MOV dword ptr [ESP + 0x188],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x5c)); // MOV EDX,dword ptr [EBX + 0x5c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x18c), self.edx); // MOV dword ptr [ESP + 0x18c],EDX
                    self.edx = self.e.mem.u32(self.ecx.wrapping_add(0x50)); // MOV EDX,dword ptr [ECX + 0x50]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x100), self.edx); // MOV dword ptr [ESP + 0x100],EDX
                    self.edx = self.e.mem.u32(self.ecx.wrapping_add(0x54)); // MOV EDX,dword ptr [ECX + 0x54]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x104), self.edx); // MOV dword ptr [ESP + 0x104],EDX
                    self.edx = self.e.mem.u32(self.ecx.wrapping_add(0x58)); // MOV EDX,dword ptr [ECX + 0x58]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x108), self.edx); // MOV dword ptr [ESP + 0x108],EDX
                    self.edx = self.e.mem.u32(self.ecx.wrapping_add(0x5c)); // MOV EDX,dword ptr [ECX + 0x5c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x10c), self.edx); // MOV dword ptr [ESP + 0x10c],EDX
                    self.fld32(self.eax.wrapping_add(0x40)); // FLD float ptr [EAX + 0x40]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.xmm[4] = load_vec(self.e, self.esp.wrapping_add(0x100)); // MOVAPS XMM4,xmmword ptr [ESP + 0x100]
                    self.x87.fld_st(0); // FLD ST0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.esp.wrapping_add(0x180)); // MOVAPS XMM2,xmmword ptr [ESP + 0x180]
                    self.x87.fmulp(2); // FMULP ST2
                    self.xmm[1] = add(self.xmm[1], self.xmm[0]); // ADDPS XMM1,XMM0
                    self.x87.fxch(1); // FXCH
                    self.xmm[0] = load_vec(self.e, self.esi); // MOVAPS XMM0,xmmword ptr [ESI]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[4]); // MULPS XMM0,XMM4
                    self.xmm[0] = add(self.xmm[0], self.xmm[2]); // ADDPS XMM0,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x2c))); // MOVSS XMM2,dword ptr [ESP + 0x2c]
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0x0); // SHUFPS XMM2,XMM2,0x0
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = self.xmm[3]; // MOVAPS XMM0,XMM3
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0x0); // SHUFPS XMM0,XMM0,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[0] = add(self.xmm[0], self.xmm[2]); // ADDPS XMM0,XMM2
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xaa); // SHUFPS XMM2,XMM0,0xaa
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[1]); // ADDSS XMM2,XMM1
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x80), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0x80],XMM2
                    self.fsub32(self.esp.wrapping_add(0x80)); // FSUB float ptr [ESP + 0x80]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x2c))); // MOVSS XMM0,dword ptr [ESP + 0x2c]
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x88), self.xmm[0][0].to_bits()); // MOVSS dword ptr [ESP + 0x88],XMM0
                    self.fld32(self.edi); // FLD float ptr [EDI]
                    self.x87.fchs(); // FCHS
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x88)); // FLD float ptr [ESP + 0x88]
                    self.x87.fld_st(0); // FLD ST0
                    self.fsub32(self.esi.wrapping_add(0x20)); // FSUB float ptr [ESI + 0x20]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esi.wrapping_add(0xc)); // FLD float ptr [ESI + 0xc]
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fcom_st(1, false); // FCOM
                    self.fnstsw_ax(); // FNSTSW AX
                    self.x87.fstp_st(1); // FSTP ST1
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d90fc9
                        block = 0x00d9_0fc9;
                        continue;
                    }
                    block = 0x00d9_0f7e;
                }
                0x00d9_0f7e => {
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.x87.fstp_st(1); // FSTP ST1
                    self.xmm[3] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [ECX + 0x30]
                    self.x87.pop(); // FSTP ST0
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[1] = self.xmm[2]; // MOVAPS XMM1,XMM2
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[2], 0x0); // SHUFPS XMM1,XMM2,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[3] = mul(self.xmm[3], self.xmm[1]); // MULPS XMM3,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi); // MOVAPS XMM1,xmmword ptr [ESI]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [ESI + 0x10]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM3,dword ptr [EDI]
                    self.xmm[0] = add(self.xmm[0], self.xmm[4]); // ADDPS XMM0,XMM4
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x20],XMM0
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ECX + 0x20]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.ecx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [ECX + 0x20],XMM0
                    block = 0x00d9_1031; // JMP 0x00d91031
                }
                0x00d9_0fc9 => {
                    self.fld32(self.esi.wrapping_add(0x24)); // FLD float ptr [ESI + 0x24]
                    self.x87.fsubp(2); // FSUBP ST2,ST0
                    self.x87.fxch(1); // FXCH
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esi.wrapping_add(0xc)); // FLD float ptr [ESI + 0xc]
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fcompp(); // FCOMPP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d91039
                        block = 0x00d9_1039;
                        continue;
                    }
                    block = 0x00d9_0fec;
                }
                0x00d9_0fec => {
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [ECX + 0x30]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[1] = self.xmm[2]; // MOVAPS XMM1,XMM2
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[2], 0x0); // SHUFPS XMM1,XMM2,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[3] = mul(self.xmm[3], self.xmm[1]); // MULPS XMM3,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi); // MOVAPS XMM1,xmmword ptr [ESI]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [ESI + 0x10]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM3,dword ptr [EDI]
                    self.xmm[0] = add(self.xmm[0], self.xmm[4]); // ADDPS XMM0,XMM4
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x20],XMM0
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ECX + 0x20]
                    self.xmm[1] = add(self.xmm[1], self.xmm[0]); // ADDPS XMM1,XMM0
                    store_vec(self.e, self.ecx.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [ECX + 0x20],XMM1
                    block = 0x00d9_1031;
                }
                0x00d9_1031 => {
                    self.xmm[3] = add_ss(self.xmm[3], self.xmm[2]); // ADDSS XMM3,XMM2
                    self.e.mem.set_u32(self.edi, self.xmm[3][0].to_bits()); // MOVSS dword ptr [EDI],XMM3
                    block = 0x00d9_1039;
                }
                0x00d9_1039 => {
                    self.push(self.esi); // PUSH ESI
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.eax = (self.eax & 0xffffff00)
                        | u32::from(self.e.mem.u8(self.esi.wrapping_add(0x30))); // MOV AL,byte ptr [ESI + 0x30]
                    self.esp = self.add32(self.esp, 0x4); // ADD ESP,0x4
                    self.esi = self.add32(self.esi, 0x30); // ADD ESI,0x30
                    self.cmp8(self.eax as u8, 0xd); // CMP AL,0xd
                    if self.zf {
                        // JZ 0x00d90e30
                        block = 0x00d9_0e30;
                        continue;
                    }
                    block = 0x00d9_1053;
                }
                0x00d9_1053 => {
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_1058 => {
                    self.eax = self.esi.wrapping_add(0x24); // LEA EAX,[ESI + 0x24]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.eax); // MOV dword ptr [ESP + 0x30],EAX
                    block = 0x00d9_105f;
                }
                0x00d9_105f => {
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV ECX,dword ptr [ESP + 0x30]
                    self.fld32(self.ecx.wrapping_add(0x4)); // FLD float ptr [ECX + 0x4]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.fmul32(self.esp.wrapping_add(0x78)); // FMUL float ptr [ESP + 0x78]
                    self.fadd32(self.edi.wrapping_add(0x8)); // FADD float ptr [EDI + 0x8]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fst32(self.esp.wrapping_add(0x10)); // FST float ptr [ESP + 0x10]
                    self.fstp32(self.edi.wrapping_add(0x8)); // FSTP float ptr [EDI + 0x8]
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x50)); // MOVAPS XMM0,xmmword ptr [EBX + 0x50]
                    self.fld32(self.ecx.wrapping_sub(0x8)); // FLD float ptr [ECX + -0x8]
                    self.xmm[3] = load_vec(self.e, self.eax.wrapping_add(0x50)); // MOVAPS XMM3,xmmword ptr [EAX + 0x50]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EBX + 0x20]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[2] = movss_load(self.e.mem.u32(self.ecx.wrapping_add(0xc))); // MOVSS XMM2,dword ptr [ECX + 0xc]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fadd32(self.esp.wrapping_add(0x10)); // FADD float ptr [ESP + 0x10]
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x50)); // MOV EDX,dword ptr [EBX + 0x50]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EAX + 0x20]
                    self.xmm[0] = sub(self.xmm[0], self.xmm[3]); // SUBPS XMM0,XMM3
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esi); // MOVAPS XMM3,xmmword ptr [ESI]
                    self.fld32(self.ecx.wrapping_add(0x8)); // FLD float ptr [ECX + 0x8]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[3] = load_vec(self.e, self.ecx.wrapping_sub(0x14)); // MOVAPS XMM3,xmmword ptr [ECX + -0x14]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xe0), self.edx); // MOV dword ptr [ESP + 0xe0],EDX
                    self.x87.fld_st(0); // FLD ST0
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x54)); // MOV EDX,dword ptr [EBX + 0x54]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xe4), self.edx); // MOV dword ptr [ESP + 0xe4],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x58)); // MOV EDX,dword ptr [EBX + 0x58]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xe8), self.edx); // MOV dword ptr [ESP + 0xe8],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x5c)); // MOV EDX,dword ptr [EBX + 0x5c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xec), self.edx); // MOV dword ptr [ESP + 0xec],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x50)); // MOV EDX,dword ptr [EAX + 0x50]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x120), self.edx); // MOV dword ptr [ESP + 0x120],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x54)); // MOV EDX,dword ptr [EAX + 0x54]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x124), self.edx); // MOV dword ptr [ESP + 0x124],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x58)); // MOV EDX,dword ptr [EAX + 0x58]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x128), self.edx); // MOV dword ptr [ESP + 0x128],EDX
                    self.eax = self.e.mem.u32(self.eax.wrapping_add(0x5c)); // MOV EAX,dword ptr [EAX + 0x5c]
                    self.edx = self.e.mem.u32(self.ebp.wrapping_add(0x8)); // MOV EDX,dword ptr [EBP + 0x8]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esp.wrapping_add(0xe0)); // MOVAPS XMM1,xmmword ptr [ESP + 0xe0]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x12c), self.eax); // MOV dword ptr [ESP + 0x12c],EAX
                    self.fmul32(self.edx.wrapping_add(0x40)); // FMUL float ptr [EDX + 0x40]
                    self.xmm[4] = load_vec(self.e, self.ecx.wrapping_sub(0x14)); // MOVAPS XMM4,xmmword ptr [ECX + -0x14]
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi); // MOVAPS XMM0,xmmword ptr [ESI]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0x120)); // MOVAPS XMM0,xmmword ptr [ESP + 0x120]
                    self.fmul32(self.esp.wrapping_add(0x10)); // FMUL float ptr [ESP + 0x10]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[4]); // MULPS XMM0,XMM4
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x2c))); // MOVSS XMM1,dword ptr [ESP + 0x2c]
                    self.eax = self.ecx.wrapping_sub(0x14); // LEA EAX,[ECX + -0x14]
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0x0); // SHUFPS XMM1,XMM1,0x0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0][0] = self.xmm[2][0]; // MOVSS XMM0,XMM2
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0x0); // SHUFPS XMM0,XMM0,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xaa); // SHUFPS XMM2,XMM0,0xaa
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[1]); // ADDSS XMM2,XMM1
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x90), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0x90],XMM2
                    self.fsub32(self.esp.wrapping_add(0x90)); // FSUB float ptr [ESP + 0x90]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x50), self.eax); // MOV dword ptr [ESP + 0x50],EAX
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x2c))); // MOVSS XMM0,dword ptr [ESP + 0x2c]
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x98), self.xmm[0][0].to_bits()); // MOVSS dword ptr [ESP + 0x98],XMM0
                    self.fld32(self.ecx.wrapping_sub(0x18)); // FLD float ptr [ECX + -0x18]
                    self.fmul32(self.esp.wrapping_add(0x98)); // FMUL float ptr [ESP + 0x98]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.edx = self.ecx.wrapping_sub(0x4); // LEA EDX,[ECX + -0x4]
                    self.fld32(self.edx); // FLD float ptr [EDX]
                    self.fcom_st(1, true); // FCOMP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d911d4
                        block = 0x00d9_11d4;
                        continue;
                    }
                    block = 0x00d9_11c7;
                }
                0x00d9_11c7 => {
                    self.fdivr32(self.edx); // FDIVR float ptr [EDX]
                    self.fstp32(self.edi.wrapping_add(0x4)); // FSTP float ptr [EDI + 0x4]
                    self.fld32(self.edx); // FLD float ptr [EDX]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    block = 0x00d9_11f3; // JMP 0x00d911f3
                }
                0x00d9_11d4 => {
                    self.fld32(self.ecx); // FLD float ptr [ECX]
                    self.fcom_st(1, true); // FCOMP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x41); // TEST AH,0x41
                    if !self.zf {
                        // JNZ 0x00d911ec
                        block = 0x00d9_11ec;
                        continue;
                    }
                    block = 0x00d9_11df;
                }
                0x00d9_11df => {
                    self.fdivr32(self.ecx); // FDIVR float ptr [ECX]
                    self.fstp32(self.edi.wrapping_add(0x4)); // FSTP float ptr [EDI + 0x4]
                    self.fld32(self.ecx); // FLD float ptr [ECX]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    block = 0x00d9_11f3; // JMP 0x00d911f3
                }
                0x00d9_11ec => {
                    self.x87.pop(); // FSTP ST0
                    self.x87.push(1.0); // FLD1
                    self.fstp32(self.edi.wrapping_add(0x4)); // FSTP float ptr [EDI + 0x4]
                    block = 0x00d9_11f3;
                }
                0x00d9_11f3 => {
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EBX + 0x30]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x50)); // MOV ECX,dword ptr [ESP + 0x50]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[2], 0x0); // SHUFPS XMM0,XMM2,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM3,xmmword ptr [EAX + 0x30]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi); // MOVAPS XMM3,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = load_vec(self.e, self.ecx); // MOVAPS XMM3,xmmword ptr [ECX]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM3,dword ptr [EDI]
                    self.xmm[4] = add(self.xmm[4], self.xmm[1]); // ADDPS XMM4,XMM1
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[4]); // MOVAPS xmmword ptr [EBX + 0x20],XMM4
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.eax.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EAX + 0x20],XMM0
                    self.xmm[3] = add_ss(self.xmm[3], self.xmm[2]); // ADDSS XMM3,XMM2
                    self.push(self.esi); // PUSH ESI
                    self.e.mem.set_u32(self.edi, self.xmm[3][0].to_bits()); // MOVSS dword ptr [EDI],XMM3
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x34)); // MOV ECX,dword ptr [ESP + 0x34]
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.eax = (self.eax & 0xffffff00)
                        | u32::from(self.e.mem.u8(self.ecx.wrapping_add(0x1c))); // MOV AL,byte ptr [ECX + 0x1c]
                    self.ecx = self.add32(self.ecx, 0x40); // ADD ECX,0x40
                    self.esp = self.add32(self.esp, 0x4); // ADD ESP,0x4
                    self.esi = self.add32(self.esi, 0x40); // ADD ESI,0x40
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.ecx); // MOV dword ptr [ESP + 0x30],ECX
                    self.cmp8(self.eax as u8, 0xf); // CMP AL,0xf
                    if self.zf {
                        // JZ 0x00d9105f
                        block = 0x00d9_105f;
                        continue;
                    }
                    block = 0x00d9_126c;
                }
                0x00d9_126c => {
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_1271 => {
                    self.xmm[2] = load_vec(self.e, self.esi); // MOVAPS XMM2,xmmword ptr [ESI]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EDX,dword ptr [ESP + 0x4c]
                    self.xmm[4] = load_vec(self.e, self.edx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EDX + 0x20]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EBX + 0x20]
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM5,xmmword ptr [EBX + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.eax = self.ebx.wrapping_add(0x20); // LEA EAX,[EBX + 0x20]
                    self.ecx = self.edx.wrapping_add(0x20); // LEA ECX,[EDX + 0x20]
                    self.xmm[0] = self.xmm[3]; // MOVAPS XMM0,XMM3
                    self.xmm[0] = mul(self.xmm[0], self.xmm[4]); // MULPS XMM0,XMM4
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[4] = shufps(self.xmm[4], self.xmm[0], 0x55); // SHUFPS XMM4,XMM0,0x55
                    self.xmm[4] = add_ss(self.xmm[4], self.xmm[0]); // ADDSS XMM4,XMM0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0xaa); // SHUFPS XMM1,XMM0,0xaa
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[4]); // ADDSS XMM1,XMM4
                    self.xmm[0] = self.xmm[3]; // MOVAPS XMM0,XMM3
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[3], 0xff); // SHUFPS XMM0,XMM3,0xff
                    self.xmm[0] = sub_ss(self.xmm[0], self.xmm[1]); // SUBSS XMM0,XMM1
                    self.xmm[1] = self.xmm[2]; // MOVAPS XMM1,XMM2
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[2], 0xff); // SHUFPS XMM1,XMM2,0xff
                    self.xmm[0] = mul_ss(self.xmm[0], self.xmm[1]); // MULSS XMM0,XMM1
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[4], 0x0); // SHUFPS XMM0,XMM4,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[5]); // MULPS XMM1,XMM5
                    self.xmm[5] = load_vec(self.e, self.edx.wrapping_add(0x30)); // MOVAPS XMM5,xmmword ptr [EDX + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[5]); // MULPS XMM0,XMM5
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.eax); // MOVAPS XMM3,xmmword ptr [EAX]
                    self.xmm[1] = add(self.xmm[1], self.xmm[3]); // ADDPS XMM1,XMM3
                    store_vec(self.e, self.eax, self.xmm[1]); // MOVAPS xmmword ptr [EAX],XMM1
                    self.xmm[1] = load_vec(self.e, self.ecx); // MOVAPS XMM1,xmmword ptr [ECX]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.ecx, self.xmm[0]); // MOVAPS xmmword ptr [ECX],XMM0
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[4]); // ADDSS XMM2,XMM4
                    self.push(self.esi); // PUSH ESI
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.eax = (self.eax & 0xffffff00)
                        | u32::from(self.e.mem.u8(self.esi.wrapping_add(0x20))); // MOV AL,byte ptr [ESI + 0x20]
                    self.esp = self.add32(self.esp, 0x4); // ADD ESP,0x4
                    self.esi = self.add32(self.esi, 0x20); // ADD ESI,0x20
                    self.cmp8(self.eax as u8, 0xc); // CMP AL,0xc
                    if self.zf {
                        // JZ 0x00d91271
                        block = 0x00d9_1271;
                        continue;
                    }
                    block = 0x00d9_1318;
                }
                0x00d9_1318 => {
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_1320 => {
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EBX + 0x10]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [ESI + 0x20]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EDX,dword ptr [ESP + 0x4c]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[0]); // SUBPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.edx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EDX + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi); // MOVAPS XMM0,xmmword ptr [ESI]
                    self.xmm[4] = mul(self.xmm[4], self.xmm[3]); // MULPS XMM4,XMM3
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[1] = add(self.xmm[1], self.xmm[4]); // ADDPS XMM1,XMM4
                    self.xmm[1] = add(self.xmm[1], self.xmm[2]); // ADDPS XMM1,XMM2
                    self.eax = self.ebx.wrapping_add(0x10); // LEA EAX,[EBX + 0x10]
                    self.ecx = self.add32(self.ecx, 0x10); // ADD ECX,0x10
                    self.edx = self.add32(self.edx, 0x20); // ADD EDX,0x20
                    self.xmm[4] = self.xmm[1]; // MOVAPS XMM4,XMM1
                    self.xmm[4] = shufps(self.xmm[4], self.xmm[1], 0x55); // SHUFPS XMM4,XMM1,0x55
                    self.xmm[4] = add_ss(self.xmm[4], self.xmm[1]); // ADDSS XMM4,XMM1
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[1], 0xaa); // SHUFPS XMM2,XMM1,0xaa
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[3], 0xff); // SHUFPS XMM3,XMM3,0xff
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0xff); // SHUFPS XMM1,XMM0,0xff
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[4]); // ADDSS XMM2,XMM4
                    self.xmm[1] = sub_ss(self.xmm[1], self.xmm[2]); // SUBSS XMM1,XMM2
                    self.xmm[1] = mul_ss(self.xmm[1], self.xmm[3]); // MULSS XMM1,XMM3
                    self.xmm[4] = self.xmm[1]; // MOVAPS XMM4,XMM1
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [EBX + 0x30]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.edx); // MOV dword ptr [ESP + 0x30],EDX
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EDX,dword ptr [ESP + 0x4c]
                    self.xmm[2] = load_vec(self.e, self.edx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EDX + 0x30]
                    self.xmm[6] = self.xmm[0]; // MOVAPS XMM6,XMM0
                    self.xmm[3] = self.xmm[4]; // MOVAPS XMM3,XMM4
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[4], 0x0); // SHUFPS XMM3,XMM4,0x0
                    self.xmm[2] = mul(self.xmm[2], self.xmm[3]); // MULPS XMM2,XMM3
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[6] = mul(self.xmm[6], self.xmm[3]); // MULPS XMM6,XMM3
                    self.xmm[3] = load_vec(self.e, self.eax); // MOVAPS XMM3,xmmword ptr [EAX]
                    self.xmm[3] = add(self.xmm[3], self.xmm[6]); // ADDPS XMM3,XMM6
                    store_vec(self.e, self.eax, self.xmm[3]); // MOVAPS xmmword ptr [EAX],XMM3
                    self.xmm[3] = load_vec(self.e, self.ecx); // MOVAPS XMM3,xmmword ptr [ECX]
                    self.xmm[5] = self.xmm[2]; // MOVAPS XMM5,XMM2
                    self.xmm[5] = shufps(self.xmm[5], self.xmm[2], 0xff); // SHUFPS XMM5,XMM2,0xff
                    self.xmm[0] = mul(self.xmm[0], self.xmm[5]); // MULPS XMM0,XMM5
                    self.xmm[3] = sub(self.xmm[3], self.xmm[0]); // SUBPS XMM3,XMM0
                    store_vec(self.e, self.ecx, self.xmm[3]); // MOVAPS xmmword ptr [ECX],XMM3
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [ESI + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.eax = self.ebx.wrapping_add(0x20); // LEA EAX,[EBX + 0x20]
                    self.xmm[3] = add(self.xmm[3], self.xmm[0]); // ADDPS XMM3,XMM0
                    store_vec(self.e, self.eax, self.xmm[3]); // MOVAPS xmmword ptr [EAX],XMM3
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EAX,dword ptr [ESP + 0x30]
                    self.xmm[0] = load_vec(self.e, self.eax); // MOVAPS XMM0,xmmword ptr [EAX]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.eax, self.xmm[0]); // MOVAPS xmmword ptr [EAX],XMM0
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[4]); // ADDSS XMM2,XMM4
                    self.push(self.esi); // PUSH ESI
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.esi = self.add32(self.esi, 0x30); // ADD ESI,0x30
                    self.eax = self.esp.wrapping_add(0x76); // LEA EAX,[ESP + 0x76]
                    self.push(self.esi); // PUSH ESI
                    self.push(self.eax); // PUSH EAX
                    self.call_copy_byte(); // CALL 0x00d921c0
                    self.eax = i32::from(self.e.mem.i8(self.eax)) as u32; // MOVSX EAX,byte ptr [EAX]
                    self.esp = self.add32(self.esp, 0xc); // ADD ESP,0xc
                    self.cmp32(self.eax, 0x5); // CMP EAX,0x5
                    if self.zf {
                        // JZ 0x00d91320
                        block = 0x00d9_1320;
                        continue;
                    }
                    block = 0x00d9_142a;
                }
                0x00d9_142a => {
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_1430 => {
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EDX,dword ptr [ESP + 0x4c]
                    self.xmm[3] = load_vec(self.e, self.edx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EDX + 0x20]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi); // MOVAPS XMM3,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[4] = mul(self.xmm[4], self.xmm[2]); // MULPS XMM4,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[4]); // ADDPS XMM0,XMM4
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.eax = self.ebx.wrapping_add(0x10); // LEA EAX,[EBX + 0x10]
                    self.ecx = self.add32(self.ecx, 0x10); // ADD ECX,0x10
                    self.edx = self.add32(self.edx, 0x20); // ADD EDX,0x20
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[3], 0xff); // SHUFPS XMM3,XMM3,0xff
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[3] = sub_ss(self.xmm[3], self.xmm[0]); // SUBSS XMM3,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM0,dword ptr [EDI]
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[3] = mul_ss(self.xmm[3], self.xmm[2]); // MULSS XMM3,XMM2
                    self.xmm[2] = load_vec(self.e, VECTOR_CONSTANT_010C7160); // MOVAPS XMM2,xmmword ptr [0x010c7160]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.edx); // MOV dword ptr [ESP + 0x30],EDX
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EDX,dword ptr [ESP + 0x4c]
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xff); // SHUFPS XMM0,XMM0,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[1]); // SUBSS XMM2,XMM1
                    self.xmm[2] = max_ps(self.xmm[2], self.xmm[3]); // MAXPS XMM2,XMM3
                    self.xmm[0] = sub_ss(self.xmm[0], self.xmm[1]); // SUBSS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [EBX + 0x30]
                    self.xmm[0] = min_ps(self.xmm[0], self.xmm[2]); // MINPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.edx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EDX + 0x30]
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[0], 0x0); // SHUFPS XMM3,XMM0,0x0
                    self.xmm[2] = mul(self.xmm[2], self.xmm[3]); // MULPS XMM2,XMM3
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[4] = self.xmm[2]; // MOVAPS XMM4,XMM2
                    self.xmm[4] = shufps(self.xmm[4], self.xmm[2], 0xff); // SHUFPS XMM4,XMM2,0xff
                    self.xmm[4] = mul(self.xmm[4], self.xmm[5]); // MULPS XMM4,XMM5
                    self.xmm[5] = load_vec(self.e, self.eax); // MOVAPS XMM5,xmmword ptr [EAX]
                    self.xmm[3] = add(self.xmm[3], self.xmm[5]); // ADDPS XMM3,XMM5
                    store_vec(self.e, self.eax, self.xmm[3]); // MOVAPS xmmword ptr [EAX],XMM3
                    self.xmm[3] = load_vec(self.e, self.ecx); // MOVAPS XMM3,xmmword ptr [ECX]
                    self.xmm[3] = sub(self.xmm[3], self.xmm[4]); // SUBPS XMM3,XMM4
                    store_vec(self.e, self.ecx, self.xmm[3]); // MOVAPS xmmword ptr [ECX],XMM3
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[4] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[1]); // MULPS XMM3,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [ESI + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.eax = self.ebx.wrapping_add(0x20); // LEA EAX,[EBX + 0x20]
                    self.xmm[4] = add(self.xmm[4], self.xmm[3]); // ADDPS XMM4,XMM3
                    store_vec(self.e, self.eax, self.xmm[4]); // MOVAPS xmmword ptr [EAX],XMM4
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x30)); // MOV EAX,dword ptr [ESP + 0x30]
                    self.xmm[3] = load_vec(self.e, self.eax); // MOVAPS XMM3,xmmword ptr [EAX]
                    self.xmm[3] = add(self.xmm[3], self.xmm[1]); // ADDPS XMM3,XMM1
                    store_vec(self.e, self.eax, self.xmm[3]); // MOVAPS xmmword ptr [EAX],XMM3
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[0]); // ADDSS XMM2,XMM0
                    self.push(self.esi); // PUSH ESI
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.esi = self.add32(self.esi, 0x30); // ADD ESI,0x30
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.eax = self.esp.wrapping_add(0x77); // LEA EAX,[ESP + 0x77]
                    self.push(self.esi); // PUSH ESI
                    self.push(self.eax); // PUSH EAX
                    self.call_copy_byte(); // CALL 0x00d921c0
                    self.eax = i32::from(self.e.mem.i8(self.eax)) as u32; // MOVSX EAX,byte ptr [EAX]
                    self.esp = self.add32(self.esp, 0xc); // ADD ESP,0xc
                    self.cmp32(self.eax, 0x6); // CMP EAX,0x6
                    if self.zf {
                        // JZ 0x00d91430
                        block = 0x00d9_1430;
                        continue;
                    }
                    block = 0x00d9_1559;
                }
                0x00d9_1559 => {
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_1560 => {
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x40)); // MOVAPS XMM1,xmmword ptr [EBX + 0x40]
                    self.fld32(self.esi.wrapping_add(0x30)); // FLD float ptr [ESI + 0x30]
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [EBX + 0x10]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x50)); // MOVAPS XMM2,xmmword ptr [EBX + 0x50]
                    self.fld32(self.esi.wrapping_add(0xc)); // FLD float ptr [ESI + 0xc]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.fstp32(self.esp.wrapping_add(0x50)); // FSTP float ptr [ESP + 0x50]
                    self.xmm[3] = load_vec(self.e, self.eax.wrapping_add(0x40)); // MOVAPS XMM3,xmmword ptr [EAX + 0x40]
                    self.xmm[4] = load_vec(self.e, self.eax.wrapping_add(0x50)); // MOVAPS XMM4,xmmword ptr [EAX + 0x50]
                    self.xmm[5] = movss_load(self.e.mem.u32(self.esi.wrapping_add(0x34))); // MOVSS XMM5,dword ptr [ESI + 0x34]
                    self.xmm[0] = sub(self.xmm[0], self.xmm[1]); // SUBPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EBX + 0x20]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[2]); // SUBPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.ecx = self.ebx.wrapping_add(0x10); // LEA ECX,[EBX + 0x10]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    self.xmm[3] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EAX + 0x20]
                    self.xmm[0] = sub(self.xmm[0], self.xmm[2]); // SUBPS XMM0,XMM2
                    self.edx = self.eax.wrapping_add(0x10); // LEA EDX,[EAX + 0x10]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x10), self.edx); // MOV dword ptr [ESP + 0x10],EDX
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.edx = self.eax.wrapping_add(0x20); // LEA EDX,[EAX + 0x20]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x30), self.edx); // MOV dword ptr [ESP + 0x30],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x40)); // MOV EDX,dword ptr [EBX + 0x40]
                    self.xmm[3] = sub(self.xmm[3], self.xmm[4]); // SUBPS XMM3,XMM4
                    self.xmm[4] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM4,xmmword ptr [ESI + 0x10]
                    self.xmm[4] = mul(self.xmm[4], self.xmm[1]); // MULPS XMM4,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi); // MOVAPS XMM1,xmmword ptr [ESI]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x160), self.edx); // MOV dword ptr [ESP + 0x160],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x44)); // MOV EDX,dword ptr [EBX + 0x44]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x164), self.edx); // MOV dword ptr [ESP + 0x164],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x48)); // MOV EDX,dword ptr [EBX + 0x48]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x168), self.edx); // MOV dword ptr [ESP + 0x168],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x4c)); // MOV EDX,dword ptr [EBX + 0x4c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x16c), self.edx); // MOV dword ptr [ESP + 0x16c],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x50)); // MOV EDX,dword ptr [EBX + 0x50]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xf0), self.edx); // MOV dword ptr [ESP + 0xf0],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x54)); // MOV EDX,dword ptr [EBX + 0x54]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xf4), self.edx); // MOV dword ptr [ESP + 0xf4],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x58)); // MOV EDX,dword ptr [EBX + 0x58]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xf8), self.edx); // MOV dword ptr [ESP + 0xf8],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x5c)); // MOV EDX,dword ptr [EBX + 0x5c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xfc), self.edx); // MOV dword ptr [ESP + 0xfc],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x40)); // MOV EDX,dword ptr [EAX + 0x40]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x140), self.edx); // MOV dword ptr [ESP + 0x140],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x44)); // MOV EDX,dword ptr [EAX + 0x44]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x144), self.edx); // MOV dword ptr [ESP + 0x144],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x48)); // MOV EDX,dword ptr [EAX + 0x48]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x148), self.edx); // MOV dword ptr [ESP + 0x148],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x4c)); // MOV EDX,dword ptr [EAX + 0x4c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x14c), self.edx); // MOV dword ptr [ESP + 0x14c],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x50)); // MOV EDX,dword ptr [EAX + 0x50]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x110), self.edx); // MOV dword ptr [ESP + 0x110],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x54)); // MOV EDX,dword ptr [EAX + 0x54]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.e.mem.set_u32(self.esp.wrapping_add(0x114), self.edx); // MOV dword ptr [ESP + 0x114],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x58)); // MOV EDX,dword ptr [EAX + 0x58]
                    self.xmm[0] = add(self.xmm[0], self.xmm[4]); // ADDPS XMM0,XMM4
                    self.xmm[2] = mul(self.xmm[2], self.xmm[1]); // MULPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, self.esp.wrapping_add(0x160)); // MOVAPS XMM1,xmmword ptr [ESP + 0x160]
                    self.xmm[0] = add(self.xmm[0], self.xmm[2]); // ADDPS XMM0,XMM2
                    self.e.mem.set_u32(self.esp.wrapping_add(0x118), self.edx); // MOV dword ptr [ESP + 0x118],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x5c)); // MOV EDX,dword ptr [EAX + 0x5c]
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0x140)); // MOVAPS XMM0,xmmword ptr [ESP + 0x140]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x11c), self.edx); // MOV dword ptr [ESP + 0x11c],EDX
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[4] = load_vec(self.e, self.esp.wrapping_add(0x110)); // MOVAPS XMM4,xmmword ptr [ESP + 0x110]
                    self.edx = self.e.mem.u32(self.ebp.wrapping_add(0x8)); // MOV EDX,dword ptr [EBP + 0x8]
                    self.fld32(self.edx.wrapping_add(0x40)); // FLD float ptr [EDX + 0x40]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0xf0)); // MOVAPS XMM0,xmmword ptr [ESP + 0xf0]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[4]); // MULPS XMM0,XMM4
                    self.xmm[4] = load_vec(self.e, self.esi); // MOVAPS XMM4,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[4]); // MULPS XMM1,XMM4
                    self.xmm[0] = add(self.xmm[0], self.xmm[2]); // ADDPS XMM0,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.push(self.esi); // PUSH ESI
                    self.x87.fld_st(0); // FLD ST0
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.fxch(1); // FXCH
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.xmm[1] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x30))); // MOVSS XMM1,dword ptr [ESP + 0x30]
                    self.fmul32(self.esp.wrapping_add(0x54)); // FMUL float ptr [ESP + 0x54]
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0x0); // SHUFPS XMM1,XMM1,0x0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = self.xmm[5]; // MOVAPS XMM0,XMM5
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0x0); // SHUFPS XMM0,XMM0,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xaa); // SHUFPS XMM2,XMM0,0xaa
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[1]); // ADDSS XMM2,XMM1
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xa4), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0xa4],XMM2
                    self.fsub32(self.esp.wrapping_add(0xa4)); // FSUB float ptr [ESP + 0xa4]
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.xmm[1] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x30))); // MOVSS XMM1,dword ptr [ESP + 0x30]
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xff); // SHUFPS XMM0,XMM0,0xff
                    self.xmm[1] = mul_ss(self.xmm[1], self.xmm[0]); // MULSS XMM1,XMM0
                    self.xmm[4] = self.xmm[1]; // MOVAPS XMM4,XMM1
                    self.xmm[0] = self.xmm[4]; // MOVAPS XMM0,XMM4
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[4], 0x0); // SHUFPS XMM0,XMM4,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EAX + 0x30]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x14)); // MOV EAX,dword ptr [ESP + 0x14]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[1], 0xff); // SHUFPS XMM2,XMM1,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[0], 0xff); // SHUFPS XMM3,XMM0,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ecx); // MOVAPS XMM5,xmmword ptr [ECX]
                    self.xmm[2] = add(self.xmm[2], self.xmm[5]); // ADDPS XMM2,XMM5
                    store_vec(self.e, self.ecx, self.xmm[2]); // MOVAPS xmmword ptr [ECX],XMM2
                    self.xmm[2] = load_vec(self.e, self.eax); // MOVAPS XMM2,xmmword ptr [EAX]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.eax, self.xmm[2]); // MOVAPS xmmword ptr [EAX],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[1]); // MULPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [ESI + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM0,dword ptr [EDI]
                    self.eax = self.ebx.wrapping_add(0x20); // LEA EAX,[EBX + 0x20]
                    self.xmm[2] = add(self.xmm[2], self.xmm[3]); // ADDPS XMM2,XMM3
                    store_vec(self.e, self.eax, self.xmm[2]); // MOVAPS xmmword ptr [EAX],XMM2
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x34)); // MOV EAX,dword ptr [ESP + 0x34]
                    self.xmm[2] = load_vec(self.e, self.eax); // MOVAPS XMM2,xmmword ptr [EAX]
                    self.xmm[2] = add(self.xmm[2], self.xmm[1]); // ADDPS XMM2,XMM1
                    store_vec(self.e, self.eax, self.xmm[2]); // MOVAPS xmmword ptr [EAX],XMM2
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[4]); // ADDSS XMM0,XMM4
                    self.e.mem.set_u32(self.edi, self.xmm[0][0].to_bits()); // MOVSS dword ptr [EDI],XMM0
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.esi = self.add32(self.esi, 0x40); // ADD ESI,0x40
                    self.eax = self.esp.wrapping_add(0x75); // LEA EAX,[ESP + 0x75]
                    self.push(self.esi); // PUSH ESI
                    self.push(self.eax); // PUSH EAX
                    self.call_copy_byte(); // CALL 0x00d921c0
                    self.eax = i32::from(self.e.mem.i8(self.eax)) as u32; // MOVSX EAX,byte ptr [EAX]
                    self.esp = self.add32(self.esp, 0xc); // ADD ESP,0xc
                    self.cmp32(self.eax, 0x7); // CMP EAX,0x7
                    if self.zf {
                        // JZ 0x00d91560
                        block = 0x00d9_1560;
                        continue;
                    }
                    block = 0x00d9_17c4;
                }
                0x00d9_17c4 => {
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_17c9 => {
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [EAX + 0x10]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [EAX + 0x20]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi); // MOVAPS XMM2,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xa8), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0xa8],XMM2
                    self.fld32(self.esi.wrapping_add(0x1c)); // FLD float ptr [ESI + 0x1c]
                    self.fmul32(self.esp.wrapping_add(0xa8)); // FMUL float ptr [ESP + 0xa8]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.x87.fld_st(0); // FLD ST0
                    self.x87.fabs(); // FABS
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fld32(self.esi.wrapping_add(0x30)); // FLD float ptr [ESI + 0x30]
                    self.fcom_st(1, true); // FCOMP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d91867
                        block = 0x00d9_1867;
                        continue;
                    }
                    block = 0x00d9_1850;
                }
                0x00d9_1850 => {
                    self.fdivr32(self.esi.wrapping_add(0x30)); // FDIVR float ptr [ESI + 0x30]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.x87.fld_st(0); // FLD ST0
                    self.x87.fmulp(2); // FMULP ST2
                    self.x87.fxch(1); // FXCH
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    block = 0x00d9_186d; // JMP 0x00d9186d
                }
                0x00d9_1867 => {
                    self.x87.pop(); // FSTP ST0
                    self.x87.pop(); // FSTP ST0
                    self.x87.push(1.0); // FLD1
                    block = 0x00d9_186d;
                }
                0x00d9_186d => {
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.fstp32(self.edi.wrapping_add(0x4)); // FSTP float ptr [EDI + 0x4]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[4], 0x0); // SHUFPS XMM0,XMM4,0x0
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EAX + 0x30]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[1], 0xff); // SHUFPS XMM2,XMM1,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[0], 0xff); // SHUFPS XMM3,XMM0,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[5]); // ADDPS XMM2,XMM5
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.eax.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EAX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[1]); // MULPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM1,dword ptr [EDI]
                    self.xmm[2] = add(self.xmm[2], self.xmm[3]); // ADDPS XMM2,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x20],XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [EAX + 0x20]
                    self.xmm[2] = add(self.xmm[2], self.xmm[0]); // ADDPS XMM2,XMM0
                    store_vec(self.e, self.eax.wrapping_add(0x20), self.xmm[2]); // MOVAPS xmmword ptr [EAX + 0x20],XMM2
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[4]); // ADDSS XMM1,XMM4
                    self.push(self.esi); // PUSH ESI
                    self.e.mem.set_u32(self.edi, self.xmm[1][0].to_bits()); // MOVSS dword ptr [EDI],XMM1
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.esi = self.add32(self.esi, 0x40); // ADD ESI,0x40
                    self.ecx = self.esp.wrapping_add(0x73); // LEA ECX,[ESP + 0x73]
                    self.push(self.esi); // PUSH ESI
                    self.push(self.ecx); // PUSH ECX
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.call_copy_byte(); // CALL 0x00d921c0
                    self.eax = i32::from(self.e.mem.i8(self.eax)) as u32; // MOVSX EAX,byte ptr [EAX]
                    self.esp = self.add32(self.esp, 0xc); // ADD ESP,0xc
                    self.cmp32(self.eax, 0x9); // CMP EAX,0x9
                    if self.zf {
                        // JZ 0x00d917c9
                        block = 0x00d9_17c9;
                        continue;
                    }
                    block = 0x00d9_1915;
                }
                0x00d9_1915 => {
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_1920 => {
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [ESI + 0x10]
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ECX + 0x20]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi); // MOVAPS XMM2,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xaa); // SHUFPS XMM0,XMM0,0xaa
                    self.xmm[0] = add_ss(self.xmm[0], self.xmm[1]); // ADDSS XMM0,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[2], 0xff); // SHUFPS XMM2,XMM2,0xff
                    self.xmm[2] = sub_ss(self.xmm[2], self.xmm[0]); // SUBSS XMM2,XMM0
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xb0), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0xb0],XMM2
                    self.fld32(self.esp.wrapping_add(0xb0)); // FLD float ptr [ESP + 0xb0]
                    self.fst32(self.esp.wrapping_add(0x30)); // FST float ptr [ESP + 0x30]
                    self.fld32(self.edi); // FLD float ptr [EDI]
                    self.x87.fchs(); // FCHS
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fsub32(self.esi.wrapping_add(0x30)); // FSUB float ptr [ESI + 0x30]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fmul32(self.esi.wrapping_add(0x1c)); // FMUL float ptr [ESI + 0x1c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fcom_st(1, false); // FCOM
                    self.fnstsw_ax(); // FNSTSW AX
                    self.x87.fstp_st(1); // FSTP ST1
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d91a2c
                        block = 0x00d9_1a2c;
                        continue;
                    }
                    block = 0x00d9_19b1;
                }
                0x00d9_19b1 => {
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [EBX + 0x30]
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [ECX + 0x30]
                    self.xmm[2] = self.xmm[4]; // MOVAPS XMM2,XMM4
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[4], 0x0); // SHUFPS XMM2,XMM4,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = self.xmm[1]; // MOVAPS XMM2,XMM1
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[1], 0xff); // SHUFPS XMM2,XMM1,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[0]; // MOVAPS XMM3,XMM0
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[0], 0xff); // SHUFPS XMM3,XMM0,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[5]); // ADDPS XMM2,XMM5
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ECX + 0x10]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [ECX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[1]); // MULPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [ESI + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM1,dword ptr [EDI]
                    self.xmm[2] = add(self.xmm[2], self.xmm[3]); // ADDPS XMM2,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x20],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ECX + 0x20]
                    self.xmm[0] = add(self.xmm[0], self.xmm[2]); // ADDPS XMM0,XMM2
                    store_vec(self.e, self.ecx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [ECX + 0x20],XMM0
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[4]); // ADDSS XMM1,XMM4
                    self.e.mem.set_u32(self.edi, self.xmm[1][0].to_bits()); // MOVSS dword ptr [EDI],XMM1
                    block = 0x00d9_1a2c;
                }
                0x00d9_1a2c => {
                    self.fld32(self.esp.wrapping_add(0x30)); // FLD float ptr [ESP + 0x30]
                    self.fsub32(self.esi.wrapping_add(0x34)); // FSUB float ptr [ESI + 0x34]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fmul32(self.esi.wrapping_add(0x1c)); // FMUL float ptr [ESI + 0x1c]
                    self.fstp32(self.esp.wrapping_add(0x10)); // FSTP float ptr [ESP + 0x10]
                    self.fld32(self.esp.wrapping_add(0x10)); // FLD float ptr [ESP + 0x10]
                    self.fcompp(); // FCOMPP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d91ace
                        block = 0x00d9_1ace;
                        continue;
                    }
                    block = 0x00d9_1a53;
                }
                0x00d9_1a53 => {
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x10))); // MOVSS XMM0,dword ptr [ESP + 0x10]
                    self.xmm[1] = load_vec(self.e, self.ecx.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [ECX + 0x30]
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[2] = self.xmm[4]; // MOVAPS XMM2,XMM4
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[4], 0x0); // SHUFPS XMM2,XMM4,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[5]); // ADDPS XMM2,XMM5
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ECX + 0x10]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.ecx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [ECX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ESI + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x20],XMM0
                    self.xmm[0] = load_vec(self.e, self.ecx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ECX + 0x20]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.ecx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [ECX + 0x20],XMM0
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[4]); // ADDSS XMM2,XMM4
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    block = 0x00d9_1ace;
                }
                0x00d9_1ace => {
                    self.push(self.esi); // PUSH ESI
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.esi = self.add32(self.esi, 0x40); // ADD ESI,0x40
                    self.edx = self.esp.wrapping_add(0x74); // LEA EDX,[ESP + 0x74]
                    self.push(self.esi); // PUSH ESI
                    self.push(self.edx); // PUSH EDX
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.call_copy_byte(); // CALL 0x00d921c0
                    self.eax = i32::from(self.e.mem.i8(self.eax)) as u32; // MOVSX EAX,byte ptr [EAX]
                    self.esp = self.add32(self.esp, 0xc); // ADD ESP,0xc
                    self.cmp32(self.eax, 0x8); // CMP EAX,0x8
                    if self.zf {
                        // JZ 0x00d91920
                        block = 0x00d9_1920;
                        continue;
                    }
                    block = 0x00d9_1af4;
                }
                0x00d9_1af4 => {
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_1af9 => {
                    self.eax = self.esi.wrapping_add(0x34); // LEA EAX,[ESI + 0x34]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x10), self.eax); // MOV dword ptr [ESP + 0x10],EAX
                    block = 0x00d9_1b00;
                }
                0x00d9_1b00 => {
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x10)); // MOV EDX,dword ptr [ESP + 0x10]
                    self.fld32(self.edx.wrapping_add(0x4)); // FLD float ptr [EDX + 0x4]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.fmul32(self.esp.wrapping_add(0x78)); // FMUL float ptr [ESP + 0x78]
                    self.ecx = self.edx.wrapping_sub(0x24); // LEA ECX,[EDX + -0x24]
                    self.edx = self.add32(self.edx, 0xffffffec); // ADD EDX,-0x14
                    self.fadd32(self.edi.wrapping_add(0x8)); // FADD float ptr [EDI + 0x8]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.fst32(self.esp.wrapping_add(0x2c)); // FST float ptr [ESP + 0x2c]
                    self.fstp32(self.edi.wrapping_add(0x8)); // FSTP float ptr [EDI + 0x8]
                    self.xmm[4] = movss_load(self.e.mem.u32(self.edx.wrapping_add(0x20))); // MOVSS XMM4,dword ptr [EDX + 0x20]
                    self.fld32(self.edx.wrapping_sub(0x14)); // FLD float ptr [EDX + -0x14]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x40)); // MOVAPS XMM1,xmmword ptr [EBX + 0x40]
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [EBX + 0x10]
                    self.fadd32(self.esp.wrapping_add(0x2c)); // FADD float ptr [ESP + 0x2c]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x50)); // MOVAPS XMM2,xmmword ptr [EBX + 0x50]
                    self.xmm[3] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [EAX + 0x10]
                    self.xmm[5] = load_vec(self.e, self.eax.wrapping_add(0x50)); // MOVAPS XMM5,xmmword ptr [EAX + 0x50]
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.fld32(self.edx.wrapping_add(0x1c)); // FLD float ptr [EDX + 0x1c]
                    self.xmm[0] = sub(self.xmm[0], self.xmm[1]); // SUBPS XMM0,XMM1
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EBX + 0x20]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[2]); // SUBPS XMM1,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x40)); // MOVAPS XMM2,xmmword ptr [EAX + 0x40]
                    self.xmm[3] = sub(self.xmm[3], self.xmm[2]); // SUBPS XMM3,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [EAX + 0x20]
                    self.xmm[0] = sub(self.xmm[0], self.xmm[3]); // SUBPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.ecx); // MOVAPS XMM3,xmmword ptr [ECX]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[3]); // MULPS XMM1,XMM3
                    self.xmm[3] = load_vec(self.e, self.edx); // MOVAPS XMM3,xmmword ptr [EDX]
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x40)); // MOV EDX,dword ptr [EBX + 0x40]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[5]); // SUBPS XMM2,XMM5
                    self.xmm[2] = mul(self.xmm[2], self.xmm[3]); // MULPS XMM2,XMM3
                    self.xmm[3] = load_vec(self.e, self.esi); // MOVAPS XMM3,xmmword ptr [ESI]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x130), self.edx); // MOV dword ptr [ESP + 0x130],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x44)); // MOV EDX,dword ptr [EBX + 0x44]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x134), self.edx); // MOV dword ptr [ESP + 0x134],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x48)); // MOV EDX,dword ptr [EBX + 0x48]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x138), self.edx); // MOV dword ptr [ESP + 0x138],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x4c)); // MOV EDX,dword ptr [EBX + 0x4c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x13c), self.edx); // MOV dword ptr [ESP + 0x13c],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x50)); // MOV EDX,dword ptr [EBX + 0x50]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x170), self.edx); // MOV dword ptr [ESP + 0x170],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x54)); // MOV EDX,dword ptr [EBX + 0x54]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x174), self.edx); // MOV dword ptr [ESP + 0x174],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x58)); // MOV EDX,dword ptr [EBX + 0x58]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x178), self.edx); // MOV dword ptr [ESP + 0x178],EDX
                    self.edx = self.e.mem.u32(self.ebx.wrapping_add(0x5c)); // MOV EDX,dword ptr [EBX + 0x5c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x17c), self.edx); // MOV dword ptr [ESP + 0x17c],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x40)); // MOV EDX,dword ptr [EAX + 0x40]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x150), self.edx); // MOV dword ptr [ESP + 0x150],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x44)); // MOV EDX,dword ptr [EAX + 0x44]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x154), self.edx); // MOV dword ptr [ESP + 0x154],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x48)); // MOV EDX,dword ptr [EAX + 0x48]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x158), self.edx); // MOV dword ptr [ESP + 0x158],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x4c)); // MOV EDX,dword ptr [EAX + 0x4c]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x15c), self.edx); // MOV dword ptr [ESP + 0x15c],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x50)); // MOV EDX,dword ptr [EAX + 0x50]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x190), self.edx); // MOV dword ptr [ESP + 0x190],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x54)); // MOV EDX,dword ptr [EAX + 0x54]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x194), self.edx); // MOV dword ptr [ESP + 0x194],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x58)); // MOV EDX,dword ptr [EAX + 0x58]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[3]); // MULPS XMM0,XMM3
                    self.xmm[3] = load_vec(self.e, self.esp.wrapping_add(0x170)); // MOVAPS XMM3,xmmword ptr [ESP + 0x170]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x198), self.edx); // MOV dword ptr [ESP + 0x198],EDX
                    self.eax = self.e.mem.u32(self.eax.wrapping_add(0x5c)); // MOV EAX,dword ptr [EAX + 0x5c]
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x10)); // MOV EDX,dword ptr [ESP + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[1]); // ADDPS XMM2,XMM1
                    self.xmm[1] = load_vec(self.e, self.esp.wrapping_add(0x130)); // MOVAPS XMM1,xmmword ptr [ESP + 0x130]
                    self.xmm[2] = add(self.xmm[2], self.xmm[0]); // ADDPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0x150)); // MOVAPS XMM0,xmmword ptr [ESP + 0x150]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x19c), self.eax); // MOV dword ptr [ESP + 0x19c],EAX
                    self.xmm[5] = load_vec(self.e, self.edx.wrapping_sub(0x14)); // MOVAPS XMM5,xmmword ptr [EDX + -0x14]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[0]); // SUBPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.ecx); // MOVAPS XMM0,xmmword ptr [ECX]
                    self.eax = self.edx.wrapping_sub(0x14); // LEA EAX,[EDX + -0x14]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0x190)); // MOVAPS XMM0,xmmword ptr [ESP + 0x190]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x50), self.eax); // MOV dword ptr [ESP + 0x50],EAX
                    self.fld32(self.esp.wrapping_add(0x2c)); // FLD float ptr [ESP + 0x2c]
                    self.eax = self.e.mem.u32(self.ebp.wrapping_add(0x8)); // MOV EAX,dword ptr [EBP + 0x8]
                    self.x87.fld_st(0); // FLD ST0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[5]); // MULPS XMM0,XMM5
                    self.fmul32(self.eax.wrapping_add(0x40)); // FMUL float ptr [EAX + 0x40]
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[5]); // MULPS XMM1,XMM5
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x2c))); // MOVSS XMM1,dword ptr [ESP + 0x2c]
                    self.fmul32(self.esp.wrapping_add(0x30)); // FMUL float ptr [ESP + 0x30]
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[1], 0x0); // SHUFPS XMM1,XMM1,0x0
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0][0] = self.xmm[4][0]; // MOVSS XMM0,XMM4
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0x0); // SHUFPS XMM0,XMM0,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xaa); // SHUFPS XMM2,XMM0,0xaa
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[1]); // ADDSS XMM2,XMM1
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0x7c), self.xmm[2][0].to_bits()); // MOVSS dword ptr [ESP + 0x7c],XMM2
                    self.fsub32(self.esp.wrapping_add(0x7c)); // FSUB float ptr [ESP + 0x7c]
                    self.eax = self.edx.wrapping_sub(0x4); // LEA EAX,[EDX + -0x4]
                    self.fstp32(self.esp.wrapping_add(0x2c)); // FSTP float ptr [ESP + 0x2c]
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x2c))); // MOVSS XMM0,dword ptr [ESP + 0x2c]
                    self.e
                        .mem
                        .set_u32(self.esp.wrapping_add(0xc0), self.xmm[0][0].to_bits()); // MOVSS dword ptr [ESP + 0xc0],XMM0
                    self.fld32(self.edx.wrapping_sub(0x18)); // FLD float ptr [EDX + -0x18]
                    self.fmul32(self.esp.wrapping_add(0xc0)); // FMUL float ptr [ESP + 0xc0]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x2c), self.eax); // MOV dword ptr [ESP + 0x2c],EAX
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.fld32(self.esp.wrapping_add(0x30)); // FLD float ptr [ESP + 0x30]
                    self.fld32(self.eax); // FLD float ptr [EAX]
                    self.fcom_st(1, true); // FCOMP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x5); // TEST AH,0x5
                    if self.pf {
                        // JP 0x00d91d03
                        block = 0x00d9_1d03;
                        continue;
                    }
                    block = 0x00d9_1cf2;
                }
                0x00d9_1cf2 => {
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x2c)); // MOV EAX,dword ptr [ESP + 0x2c]
                    self.fdivr32(self.eax); // FDIVR float ptr [EAX]
                    self.fstp32(self.edi.wrapping_add(0x4)); // FSTP float ptr [EDI + 0x4]
                    self.fld32(self.eax); // FLD float ptr [EAX]
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    block = 0x00d9_1d22; // JMP 0x00d91d22
                }
                0x00d9_1d03 => {
                    self.fld32(self.edx); // FLD float ptr [EDX]
                    self.fcom_st(1, true); // FCOMP
                    self.fnstsw_ax(); // FNSTSW AX
                    self.test8((self.eax >> 8) as u8, 0x41); // TEST AH,0x41
                    if !self.zf {
                        // JNZ 0x00d91d1b
                        block = 0x00d9_1d1b;
                        continue;
                    }
                    block = 0x00d9_1d0e;
                }
                0x00d9_1d0e => {
                    self.fdivr32(self.edx); // FDIVR float ptr [EDX]
                    self.fstp32(self.edi.wrapping_add(0x4)); // FSTP float ptr [EDI + 0x4]
                    self.fld32(self.edx); // FLD float ptr [EDX]
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    block = 0x00d9_1d22; // JMP 0x00d91d22
                }
                0x00d9_1d1b => {
                    self.x87.pop(); // FSTP ST0
                    self.x87.push(1.0); // FLD1
                    self.fstp32(self.edi.wrapping_add(0x4)); // FSTP float ptr [EDI + 0x4]
                    block = 0x00d9_1d22;
                }
                0x00d9_1d22 => {
                    self.xmm[0] = movss_load(self.e.mem.u32(self.esp.wrapping_add(0x30))); // MOVSS XMM0,dword ptr [ESP + 0x30]
                    self.xmm[2] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EBX + 0x30]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.xmm[5] = load_vec(self.e, self.esi); // MOVAPS XMM5,xmmword ptr [ESI]
                    self.xmm[4] = self.xmm[0]; // MOVAPS XMM4,XMM0
                    self.xmm[1] = self.xmm[4]; // MOVAPS XMM1,XMM4
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[4], 0x0); // SHUFPS XMM1,XMM4,0x0
                    self.xmm[0] = self.xmm[1]; // MOVAPS XMM0,XMM1
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM2,xmmword ptr [EAX + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[5]); // MULPS XMM2,XMM5
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[3] = mul(self.xmm[3], self.xmm[5]); // MULPS XMM3,XMM5
                    self.xmm[5] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[5]); // ADDPS XMM2,XMM5
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EBX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[3]); // SUBPS XMM2,XMM3
                    store_vec(self.e, self.eax.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EAX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx); // MOVAPS XMM2,xmmword ptr [ECX]
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x50)); // MOV ECX,dword ptr [ESP + 0x50]
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [EBX + 0x20]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[2] = load_vec(self.e, self.ecx); // MOVAPS XMM2,xmmword ptr [ECX]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[2] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM2,dword ptr [EDI]
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x20],XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EAX + 0x20]
                    self.xmm[1] = add(self.xmm[1], self.xmm[0]); // ADDPS XMM1,XMM0
                    store_vec(self.e, self.eax.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [EAX + 0x20],XMM1
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[4]); // ADDSS XMM2,XMM4
                    self.push(self.esi); // PUSH ESI
                    self.e.mem.set_u32(self.edi, self.xmm[2][0].to_bits()); // MOVSS dword ptr [EDI],XMM2
                    self.call_table_byte(); // CALL 0x00d921d0
                    self.edx = self.e.mem.u32(self.esp.wrapping_add(0x14)); // MOV EDX,dword ptr [ESP + 0x14]
                    self.edx = self.add32(self.edx, 0x1c); // ADD EDX,0x1c
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(4)); // LEA EDI,[EDI + EAX*0x4]
                    self.push(self.edx); // PUSH EDX
                    self.eax = self.esp.wrapping_add(0x76); // LEA EAX,[ESP + 0x76]
                    self.push(self.eax); // PUSH EAX
                    self.call_copy_byte(); // CALL 0x00d921c0
                    self.eax = i32::from(self.e.mem.i8(self.eax)) as u32; // MOVSX EAX,byte ptr [EAX]
                    let at = self.esp.wrapping_add(0x1c); // ADD dword ptr [ESP + 0x1c],0x50
                    let value = self.add32(self.e.mem.u32(at), 0x50);
                    self.e.mem.set_u32(at, value);
                    self.esp = self.add32(self.esp, 0xc); // ADD ESP,0xc
                    self.esi = self.add32(self.esi, 0x50); // ADD ESI,0x50
                    self.cmp32(self.eax, 0xa); // CMP EAX,0xa
                    if self.zf {
                        // JZ 0x00d91b00
                        block = 0x00d9_1b00;
                        continue;
                    }
                    block = 0x00d9_1dd8;
                }
                0x00d9_1dd8 => {
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_1ddd => {
                    self.xmm[0] = load_vec(self.e, self.esi); // MOVAPS XMM0,xmmword ptr [ESI]
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [ESI + 0x10]
                    self.xmm[3] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM3,xmmword ptr [ESI + 0x20]
                    self.xmm[4] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM4,xmmword ptr [EAX + 0x20]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[0]); // MULPS XMM1,XMM0
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [EAX + 0x10]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [EBX + 0x20]
                    self.xmm[3] = mul(self.xmm[3], self.xmm[0]); // MULPS XMM3,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [ESI + 0x30]
                    self.xmm[1] = sub(self.xmm[1], self.xmm[2]); // SUBPS XMM1,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[4]); // MULPS XMM0,XMM4
                    self.xmm[0] = add(self.xmm[0], self.xmm[3]); // ADDPS XMM0,XMM3
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = shufps(self.xmm[1], self.xmm[0], 0x55); // SHUFPS XMM1,XMM0,0x55
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[1] = add_ss(self.xmm[1], self.xmm[0]); // ADDSS XMM1,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xaa); // SHUFPS XMM2,XMM0,0xaa
                    self.ecx = self.esp.wrapping_add(0x270); // LEA ECX,[ESP + 0x270]
                    self.push(self.ecx); // PUSH ECX
                    self.edx = self.esp.wrapping_add(0x214); // LEA EDX,[ESP + 0x214]
                    self.xmm[2] = add_ss(self.xmm[2], self.xmm[1]); // ADDSS XMM2,XMM1
                    store_vec(self.e, self.esp.wrapping_add(0x274), self.xmm[2]); // MOVAPS xmmword ptr [ESP + 0x274],XMM2
                    self.xmm[0] = load_vec(self.e, self.esi); // MOVAPS XMM0,xmmword ptr [ESI]
                    self.push(self.edx); // PUSH EDX
                    self.eax = self.esp.wrapping_add(0x238); // LEA EAX,[ESP + 0x238]
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xff); // SHUFPS XMM0,XMM0,0xff
                    self.push(self.eax); // PUSH EAX
                    store_vec(self.e, self.esp.wrapping_add(0x21c), self.xmm[0]); // MOVAPS xmmword ptr [ESP + 0x21c],XMM0
                    self.call_cdecl_returning_first(VECTOR_SUBTRACT_FIRST_LANE, 3); // CALL 0x00c6ccc0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM0,xmmword ptr [ESI + 0x20]
                    self.ecx = self.esp.wrapping_add(0x1ec); // LEA ECX,[ESP + 0x1ec]
                    self.push(self.ecx); // PUSH ECX
                    self.edx = self.esp.wrapping_add(0x240); // LEA EDX,[ESP + 0x240]
                    self.push(self.edx); // PUSH EDX
                    self.eax = self.esp.wrapping_add(0x1e4); // LEA EAX,[ESP + 0x1e4]
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0xff); // SHUFPS XMM0,XMM0,0xff
                    self.push(self.eax); // PUSH EAX
                    store_vec(self.e, self.esp.wrapping_add(0x1f8), self.xmm[0]); // MOVAPS xmmword ptr [ESP + 0x1f8],XMM0
                    self.call_cdecl_returning_first(VECTOR_MULTIPLY_FIRST_LANE, 3); // CALL 0x00891060
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0x1e8)); // MOVAPS XMM0,xmmword ptr [ESP + 0x1e8]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x64)); // MOV EAX,dword ptr [ESP + 0x64]
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x30)); // MOVAPS XMM1,xmmword ptr [EAX + 0x30]
                    self.xmm[4] = load_vec(self.e, self.esi); // MOVAPS XMM4,xmmword ptr [ESI]
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0x0); // SHUFPS XMM0,XMM0,0x0
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [EBX + 0x30]
                    self.xmm[1] = mul(self.xmm[1], self.xmm[2]); // MULPS XMM1,XMM2
                    self.xmm[0] = mul(self.xmm[0], self.xmm[2]); // MULPS XMM0,XMM2
                    self.xmm[3] = self.xmm[1]; // MOVAPS XMM3,XMM1
                    self.xmm[3] = shufps(self.xmm[3], self.xmm[1], 0xff); // SHUFPS XMM3,XMM1,0xff
                    self.xmm[2] = self.xmm[0]; // MOVAPS XMM2,XMM0
                    self.xmm[2] = shufps(self.xmm[2], self.xmm[0], 0xff); // SHUFPS XMM2,XMM0,0xff
                    self.xmm[2] = mul(self.xmm[2], self.xmm[4]); // MULPS XMM2,XMM4
                    self.xmm[4] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM4,xmmword ptr [ESI + 0x10]
                    self.xmm[4] = mul(self.xmm[4], self.xmm[3]); // MULPS XMM4,XMM3
                    self.xmm[3] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM3,xmmword ptr [EBX + 0x10]
                    self.xmm[3] = add(self.xmm[3], self.xmm[2]); // ADDPS XMM3,XMM2
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[3]); // MOVAPS xmmword ptr [EBX + 0x10],XMM3
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[2] = sub(self.xmm[2], self.xmm[4]); // SUBPS XMM2,XMM4
                    store_vec(self.e, self.eax.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EAX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.esi.wrapping_add(0x20)); // MOVAPS XMM2,xmmword ptr [ESI + 0x20]
                    self.xmm[2] = mul(self.xmm[2], self.xmm[0]); // MULPS XMM2,XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x30)); // MOVAPS XMM0,xmmword ptr [ESI + 0x30]
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = movss_load(self.e.mem.u32(self.edi)); // MOVSS XMM1,dword ptr [EDI]
                    self.ecx = self.esp.wrapping_add(0x1e8); // LEA ECX,[ESP + 0x1e8]
                    self.push(self.ecx); // PUSH ECX
                    self.edx = self.esp.wrapping_add(0xec); // LEA EDX,[ESP + 0xec]
                    self.push(self.edx); // PUSH EDX
                    store_vec(self.e, self.esp.wrapping_add(0xf0), self.xmm[1]); // MOVAPS xmmword ptr [ESP + 0xf0],XMM1
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EBX + 0x20]
                    self.xmm[1] = add(self.xmm[1], self.xmm[2]); // ADDPS XMM1,XMM2
                    store_vec(self.e, self.ebx.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [EBX + 0x20],XMM1
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x20)); // MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    self.xmm[1] = add(self.xmm[1], self.xmm[0]); // ADDPS XMM1,XMM0
                    store_vec(self.e, self.eax.wrapping_add(0x20), self.xmm[1]); // MOVAPS xmmword ptr [EAX + 0x20],XMM1
                    self.eax = self.esp.wrapping_add(0x2a0); // LEA EAX,[ESP + 0x2a0]
                    self.push(self.eax); // PUSH EAX
                    self.call_cdecl_returning_first(VECTOR_ADD_FIRST_LANE, 3); // CALL 0x00c6cce0
                    self.ecx = self.e.mem.u32(self.eax); // MOV ECX,dword ptr [EAX]
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0x4)); // MOV EDX,dword ptr [EAX + 0x4]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xf4), self.ecx); // MOV dword ptr [ESP + 0xf4],ECX
                    self.ecx = self.e.mem.u32(self.eax.wrapping_add(0x8)); // MOV ECX,dword ptr [EAX + 0x8]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xf8), self.edx); // MOV dword ptr [ESP + 0xf8],EDX
                    self.edx = self.e.mem.u32(self.eax.wrapping_add(0xc)); // MOV EDX,dword ptr [EAX + 0xc]
                    self.e.mem.set_u32(self.esp.wrapping_add(0xfc), self.ecx); // MOV dword ptr [ESP + 0xfc],ECX
                    self.e.mem.set_u32(self.esp.wrapping_add(0x100), self.edx); // MOV dword ptr [ESP + 0x100],EDX
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0xf4)); // MOVAPS XMM0,xmmword ptr [ESP + 0xf4]
                    self.e.mem.set_u32(self.edi, self.xmm[0][0].to_bits()); // MOVSS dword ptr [EDI],XMM0
                    self.esp = self.add32(self.esp, 0x24); // ADD ESP,0x24
                    self.edi = self.add32(self.edi, 0x4); // ADD EDI,0x4
                    self.esi = self.add32(self.esi, 0x40); // ADD ESI,0x40
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_1f6c => {
                    self.xmm[0] = load_vec(self.e, self.esi); // MOVAPS XMM0,xmmword ptr [ESI]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    store_vec(self.e, self.ebx.wrapping_add(0x30), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x30],XMM0
                    self.xmm[0] = load_vec(self.e, self.esi.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [ESI + 0x10]
                    store_vec(self.e, self.eax.wrapping_add(0x30), self.xmm[0]); // MOVAPS xmmword ptr [EAX + 0x30],XMM0
                    self.esi = self.add32(self.esi, 0x20); // ADD ESI,0x20
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_1f87 => {
                    self.ecx = self.e.mem.u32(self.esi.wrapping_add(0x10)); // MOV ECX,dword ptr [ESI + 0x10]
                    self.eax = self.ebx; // MOV EAX,EBX
                    self.eax = self.xor32(self.eax, self.e.mem.u32(self.esp.wrapping_add(0x4c))); // XOR EAX,dword ptr [ESP + 0x4c]
                    self.ecx = self.neg32(self.ecx); // NEG ECX
                    self.eax = self.and32(self.eax, self.ecx); // AND EAX,ECX
                    self.ecx = self.e.mem.u32(self.ebp.wrapping_add(0x8)); // MOV ECX,dword ptr [EBP + 0x8]
                    self.fld32(self.ecx.wrapping_add(0x30)); // FLD float ptr [ECX + 0x30]
                    self.push(self.ecx); // PUSH ECX
                    self.fimul32(self.esp.wrapping_add(0x78)); // FIMUL dword ptr [ESP + 0x78]
                    self.eax = self.xor32(self.eax, self.ebx); // XOR EAX,EBX
                    self.ecx = self.esp.wrapping_add(0x1f4); // LEA ECX,[ESP + 0x1f4]
                    self.e.mem.set_u32(self.esp.wrapping_add(0x14), self.eax); // MOV dword ptr [ESP + 0x14],EAX
                    self.fstp32(self.esp.wrapping_add(0x30)); // FSTP float ptr [ESP + 0x30]
                    self.fld32(self.esp.wrapping_add(0x30)); // FLD float ptr [ESP + 0x30]
                    self.fstp32(self.esp); // FSTP float ptr [ESP]
                    self.call_this(VECTOR_FROM_FLOAT, 1); // CALL 0x005dbf20
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x10)); // MOV EAX,dword ptr [ESP + 0x10]
                    self.xmm[1] = load_vec(self.e, self.esi); // MOVAPS XMM1,xmmword ptr [ESI]
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0x1f0)); // MOVAPS XMM0,xmmword ptr [ESP + 0x1f0]
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[2] = add(self.xmm[2], self.xmm[1]); // ADDPS XMM2,XMM1
                    store_vec(self.e, self.eax.wrapping_add(0x10), self.xmm[2]); // MOVAPS xmmword ptr [EAX + 0x10],XMM2
                    self.xmm[2] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0x0); // SHUFPS XMM0,XMM0,0x0
                    self.xmm[0] = mul(self.xmm[0], self.xmm[1]); // MULPS XMM0,XMM1
                    self.xmm[1] = self.xmm[0]; // MOVAPS XMM1,XMM0
                    self.xmm[1] = add(self.xmm[1], self.xmm[2]); // ADDPS XMM1,XMM2
                    store_vec(self.e, self.eax.wrapping_add(0x10), self.xmm[1]); // MOVAPS xmmword ptr [EAX + 0x10],XMM1
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x40)); // MOVAPS XMM1,xmmword ptr [EAX + 0x40]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.eax.wrapping_add(0x40), self.xmm[0]); // MOVAPS xmmword ptr [EAX + 0x40],XMM0
                    self.esi = self.add32(self.esi, 0x20); // ADD ESI,0x20
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_1ffe => {
                    self.edx = self.ebx.wrapping_add(0x20); // LEA EDX,[EBX + 0x20]
                    self.push(self.edx); // PUSH EDX
                    self.push(self.esi); // PUSH ESI
                    self.ecx = self.esp.wrapping_add(0x208); // LEA ECX,[ESP + 0x208]
                    self.call_this(SET_MUL3, 2); // CALL 0x00c7f750
                    self.ecx = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV ECX,dword ptr [ESP + 0x4c]
                    self.ecx = self.add32(self.ecx, 0x20); // ADD ECX,0x20
                    self.push(self.ecx); // PUSH ECX
                    self.eax = self.esi.wrapping_add(0x30); // LEA EAX,[ESI + 0x30]
                    self.push(self.eax); // PUSH EAX
                    self.ecx = self.esp.wrapping_add(0x248); // LEA ECX,[ESP + 0x248]
                    self.call_this(SET_MUL3, 2); // CALL 0x00c7f750
                    self.edx = self.ebx.wrapping_add(0x50); // LEA EDX,[EBX + 0x50]
                    self.push(self.edx); // PUSH EDX
                    self.push(self.esi); // PUSH ESI
                    self.ecx = self.esp.wrapping_add(0x228); // LEA ECX,[ESP + 0x228]
                    self.call_this(SET_MUL3, 2); // CALL 0x00c7f750
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.eax = self.add32(self.eax, 0x50); // ADD EAX,0x50
                    self.push(self.eax); // PUSH EAX
                    self.eax = self.esi.wrapping_add(0x30); // LEA EAX,[ESI + 0x30]
                    self.push(self.eax); // PUSH EAX
                    self.ecx = self.esp.wrapping_add(0x268); // LEA ECX,[ESP + 0x268]
                    self.call_this(SET_MUL3, 2); // CALL 0x00c7f750
                    self.xmm[1] = load_vec(self.e, self.ebx.wrapping_add(0x10)); // MOVAPS XMM1,xmmword ptr [EBX + 0x10]
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0x200)); // MOVAPS XMM0,xmmword ptr [ESP + 0x200]
                    self.eax = self.e.mem.u32(self.esp.wrapping_add(0x4c)); // MOV EAX,dword ptr [ESP + 0x4c]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.ebx.wrapping_add(0x10), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x10],XMM0
                    self.xmm[1] = load_vec(self.e, self.esp.wrapping_add(0x220)); // MOVAPS XMM1,xmmword ptr [ESP + 0x220]
                    self.xmm[0] = load_vec(self.e, self.ebx.wrapping_add(0x40)); // MOVAPS XMM0,xmmword ptr [EBX + 0x40]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.ebx.wrapping_add(0x40), self.xmm[0]); // MOVAPS xmmword ptr [EBX + 0x40],XMM0
                    self.xmm[1] = load_vec(self.e, self.esp.wrapping_add(0x240)); // MOVAPS XMM1,xmmword ptr [ESP + 0x240]
                    self.xmm[0] = load_vec(self.e, self.eax.wrapping_add(0x10)); // MOVAPS XMM0,xmmword ptr [EAX + 0x10]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.eax.wrapping_add(0x10), self.xmm[0]); // MOVAPS xmmword ptr [EAX + 0x10],XMM0
                    self.xmm[0] = load_vec(self.e, self.esp.wrapping_add(0x260)); // MOVAPS XMM0,xmmword ptr [ESP + 0x260]
                    self.xmm[1] = load_vec(self.e, self.eax.wrapping_add(0x40)); // MOVAPS XMM1,xmmword ptr [EAX + 0x40]
                    self.xmm[0] = add(self.xmm[0], self.xmm[1]); // ADDPS XMM0,XMM1
                    store_vec(self.e, self.eax.wrapping_add(0x40), self.xmm[0]); // MOVAPS xmmword ptr [EAX + 0x40],XMM0
                    self.esi = self.add32(self.esi, 0x60); // ADD ESI,0x60
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_20a8 => {
                    self.ecx = self.e.mem.u32(self.ebp.wrapping_add(0x18)); // MOV ECX,dword ptr [EBP + 0x18]
                    self.eax = self.e.mem.u32(self.ebp.wrapping_add(0x8)); // MOV EAX,dword ptr [EBP + 0x8]
                    self.push(self.edi); // PUSH EDI
                    self.push(self.esi); // PUSH ESI
                    self.push(self.ecx); // PUSH ECX
                    self.edx = self.esp.wrapping_add(0x80); // LEA EDX,[ESP + 0x80]
                    self.push(self.edx); // PUSH EDX
                    self.push(self.eax); // PUSH EAX
                    self.call_stiff_spring_chain(); // CALL 0x00d893a0
                    self.ecx = u32::from(self.e.mem.u16(self.esi.wrapping_add(0x2))); // MOVZX ECX,word ptr [ESI + 0x2]
                    self.esp = self.add32(self.esp, 0x14); // ADD ESP,0x14
                    self.esi = self.add32(self.esi, self.e.mem.u32(self.esi.wrapping_add(0x4))); // ADD ESI,dword ptr [ESI + 0x4]
                    self.edi = self.edi.wrapping_add(self.ecx.wrapping_mul(4)); // LEA EDI,[EDI + ECX*0x4]
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_20d1 => {
                    self.edx = self.e.mem.u32(self.ebp.wrapping_add(0x18)); // MOV EDX,dword ptr [EBP + 0x18]
                    self.ecx = self.e.mem.u32(self.ebp.wrapping_add(0x8)); // MOV ECX,dword ptr [EBP + 0x8]
                    self.push(self.edi); // PUSH EDI
                    self.push(self.esi); // PUSH ESI
                    self.push(self.edx); // PUSH EDX
                    self.eax = self.esp.wrapping_add(0x80); // LEA EAX,[ESP + 0x80]
                    self.push(self.eax); // PUSH EAX
                    self.push(self.ecx); // PUSH ECX
                    self.call_ball_and_socket_chain(); // CALL 0x00d89690
                    self.eax = u32::from(self.e.mem.u16(self.esi.wrapping_add(0x2))); // MOVZX EAX,word ptr [ESI + 0x2]
                    self.edx = self.eax.wrapping_add(self.eax.wrapping_mul(2)); // LEA EDX,[EAX + EAX*0x2]
                    self.esp = self.add32(self.esp, 0x14); // ADD ESP,0x14
                    self.edi = self.edi.wrapping_add(self.edx.wrapping_mul(4)); // LEA EDI,[EDI + EDX*0x4]
                    block = 0x00d9_20f5;
                }
                0x00d9_20f5 => {
                    self.esi = self.add32(self.esi, self.e.mem.u32(self.esi.wrapping_add(0x4))); // ADD ESI,dword ptr [ESI + 0x4]
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_20fd => {
                    self.eax = self.e.mem.u32(self.ebp.wrapping_add(0x18)); // MOV EAX,dword ptr [EBP + 0x18]
                    self.edx = self.e.mem.u32(self.ebp.wrapping_add(0x8)); // MOV EDX,dword ptr [EBP + 0x8]
                    self.push(self.edi); // PUSH EDI
                    self.push(self.esi); // PUSH ESI
                    self.push(self.eax); // PUSH EAX
                    self.ecx = self.esp.wrapping_add(0x80); // LEA ECX,[ESP + 0x80]
                    self.push(self.ecx); // PUSH ECX
                    self.push(self.edx); // PUSH EDX
                    self.call_powered_chain(); // CALL 0x00d8a3a0
                    self.eax = u32::from(self.e.mem.u16(self.esi.wrapping_add(0x4))); // MOVZX EAX,word ptr [ESI + 0x4]
                    self.eax = self.eax.wrapping_add(self.eax.wrapping_mul(2)); // LEA EAX,[EAX + EAX*0x2]
                    self.esp = self.add32(self.esp, 0x14); // ADD ESP,0x14
                    self.esi = self.add32(self.esi, self.e.mem.u32(self.esi.wrapping_add(0x10))); // ADD ESI,dword ptr [ESI + 0x10]
                    self.edi = self.edi.wrapping_add(self.eax.wrapping_mul(8)); // LEA EDI,[EDI + EAX*0x8]
                    block = 0x00d8_fbfc; // JMP 0x00d8fbfc
                }
                0x00d9_2129 => {
                    panic!("INT3 at 00d92129"); // INT3
                }
                0x00d9_212a => {
                    self.edi = self.pop(); // POP EDI
                    self.esi = self.pop(); // POP ESI
                    self.eax = 0x1; // MOV EAX,0x1
                    self.ebx = self.pop(); // POP EBX
                    self.esp = self.ebp; // MOV ESP,EBP
                    self.ebp = self.pop(); // POP EBP
                    return self.eax; // RET
                }
                _ => unreachable!("no block starts at {block:08x}"),
            }
        }
    }
}

// Translated from 00d8aac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolveStepJacobians` (engine map; cdecl, four words, no answer): walks the schema of one
/// solver step and applies each record to the two bodies it currently selects.
///
/// `solver_info` is the step's parameter block, `schema` the record list, `bodies` the body
/// array (`0x80` bytes per body: the linear velocity at `+0x10`, the angular velocity at `+0x20`,
/// the inverse inertia with the inverse mass in the last lane at `+0x30`, the two pseudo velocity
/// vectors at `+0x40` and `+0x50`) and `accumulators` the float array the rows add their
/// impulses to.
///
/// Every record starts with its type byte (a Jacobian row keeps it in the low byte of its first
/// float). `0` ends the walk; `4`, `28` and everything above `31` run into a breakpoint (a panic here). `1` (`0x10`
/// bytes) selects the bodies with the 16-bit indices at `+4` and `+6`, `2` skips the number of
/// bytes at `+4`, `3` skips `0x10` bytes. The types `5` to `27` are Jacobian rows of
/// `0x30, 0x30, 0x40, 0x40, 0x40, 0x50, 0x40, 0x20, 0x30, 0x30, 0x40, 0x30, 0x30, 0x30, 0x30,
/// 0x70, 0x70, 0x70, 0x90, 0x50, 0x20, 0x20, 0x60` bytes; after a row the accumulator pointer
/// moves by the entry of the byte table at `011b9434` for the type (`1` float for most, `2` for
/// types 9, 14, 20 and 21, `3` for 10, 15, 22 and 24, `4` for 23, none for 25 to 27). Rows of
/// one type follow each other without a new dispatch, and the types 20 to 22 continue into 23
/// when the next record is one. `29` and `30` hand the record to the stiff spring and the
/// ball-and-socket chain solver (record size at `+4`, count at `+2`; the accumulators advance by
/// the count, respectively three times the count), `31` to the powered chain solver (record
/// size at `+0x10`, count at `+4`; the accumulators advance by six times the count).
pub fn hk_solve_step_jacobians(
    e: &mut Engine,
    solver_info: u32,
    schema: u32,
    bodies: u32,
    accumulators: u32,
) {
    e.with_stack(SOLVER_STACK_SIZE, |e, stack| {
        SolverMachine::new(
            e,
            stack.addr(),
            &[solver_info, schema, bodies, accumulators],
        )
        .run_step_jacobians();
    });
}

// Translated from 00d8fb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolveConstraintsByTheSteps` (engine map; cdecl, six words): the same schema walk as
/// `hkSolveStepJacobians` (`00d8aac0`) with two more inputs. It first makes sure the Havok
/// license flag (`01269204`) is set, running the check `00df22f0` once when it is not, and
/// answers `0` without touching anything when it is still clear; otherwise it answers `1`.
///
/// `step_word` and a float chosen by `selector` (`0.0` when it is not zero, `1.0` when it is)
/// form an eight byte block in the frame; the chain solvers receive its address (the powered
/// chain solver reads the float at `+4`), record type 26 multiplies the word, as a 32-bit
/// integer, with the float at `solver_info + 0x30`, and the row types 10 and 15 multiply a
/// coefficient by the float. The first function keeps `0` and `0.0` in that block.
pub fn hk_solve_constraints_by_the_steps(
    e: &mut Engine,
    solver_info: u32,
    step_word: u32,
    selector: u32,
    schema: u32,
    bodies: u32,
    accumulators: u32,
) -> u32 {
    e.with_stack(SOLVER_STACK_SIZE, |e, stack| {
        SolverMachine::new(
            e,
            stack.addr(),
            &[
                solver_info,
                step_word,
                selector,
                schema,
                bodies,
                accumulators,
            ],
        )
        .run_by_the_steps()
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d8_92d0, hk_solve_upload()),
        entry!(0x00d8_92f0, fn_00d892f0(Ptr, Ptr, Ptr, Ptr, u8)),
        entry!(
            0x00d8_93a0,
            hk_solver_solve_stiff_spring_chain(u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00d8_9690,
            hk_solver_solve_ball_and_socket_chain(u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00d8_a3a0,
            hk_solver_solve_powered_chain(u32, u32, u32, u32, u32)
        ),
        entry!(0x00d8_9ed0, fn_00d89ed0(u32, u32, u32)),
        entry!(0x00d8_9f80, fn_00d89f80(u32, i32, u32, u32, u32) -> u32),
        entry!(0x00d9_21c0, fn_00d921c0(u32, u32)),
        entry!(0x00d9_21d0, fn_00d921d0(u32) -> u8),
        entry!(0x00d8_aac0, hk_solve_step_jacobians(u32, u32, u32, u32)),
        entry!(
            0x00d8_fb70,
            hk_solve_constraints_by_the_steps(u32, u32, u32, u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn put(e: &mut Engine, at: u32, v: Vec4) {
        store_vec(e, at, v);
    }

    #[test]
    fn upload_does_nothing() {
        let mut e = engine();
        e.call(0x00d892d0, &args![]);
    }

    /// A body block (0x60 bytes) with the step in +8 and recognisable vectors.
    fn body(e: &mut Engine) -> u32 {
        let body = e.mem.alloc(0x80);
        e.mem.set_f32(body + 8, 0.5);
        put(e, body + 0x10, [1.0, 2.0, 3.0, 4.0]);
        put(e, body + 0x20, [5.0, 6.0, 7.0, 8.0]);
        put(e, body + 0x40, [0.5, 0.5, 0.5, 0.5]);
        put(e, body + 0x50, [1.0, 1.0, 1.0, 1.0]);
        body
    }

    #[test]
    fn body_integration_follows_the_damping_byte() {
        let mut e = engine();
        let gravity = e.mem.alloc(0x10);
        put(&mut e, gravity, [0.0, -10.0, 0.0, 0.0]);
        let damping = e.mem.alloc(0x10);
        put(&mut e, damping, [0.5, 0.25, 2.0, 1.0]);
        let input = e.mem.alloc(0x40);
        e.mem.set_f32(input + 0x30, 0.5);
        // No damping: positions move by half the difference, velocities gain gravity.
        let first = body(&mut e);
        e.call(0x00d892f0, &args![first, damping, gravity, input, 0u8]);
        assert_eq!(load_vec(&e, first + 0x40), [0.75, 1.25, 1.75, 2.25]);
        assert_eq!(load_vec(&e, first + 0x50), [3.0, 3.5, 4.0, 4.5]);
        assert_eq!(load_vec(&e, first + 0x20), [7.0, 8.5, 10.0, 11.5]);
        assert_eq!(load_vec(&e, first + 0x10), [1.25, -2.25, 4.25, 5.75]);
        // Damping: the two position vectors are scaled, the velocities restored.
        let second = body(&mut e);
        e.call(0x00d892f0, &args![second, damping, gravity, input, 1u8]);
        assert_eq!(load_vec(&e, second + 0x40), [0.375, 0.3125, 3.5, 2.25]);
        assert_eq!(load_vec(&e, second + 0x50), [1.5, 0.875, 8.0, 4.5]);
        assert_eq!(load_vec(&e, second + 0x20), [4.0, 5.0, 6.0, 7.0]);
        assert_eq!(load_vec(&e, second + 0x10), [0.5, 1.5, 2.5, 3.5]);
    }

    #[test]
    fn stiff_spring_chain_solves_one_link() {
        let mut e = engine();
        // One link: Jacobian rows, coefficients, solution slot and body offsets.
        let chain = e.mem.alloc(0x100);
        e.mem.set_u16(chain + 2, 1);
        e.mem.set_f32(chain + 8, 2.0);
        e.mem.set_f32(chain + 0xc, 3.0);
        put(&mut e, chain + 0x10, [1.0, 0.0, 0.0, 0.5]);
        put(&mut e, chain + 0x20, [0.0, 1.0, 0.0, 0.0]);
        put(&mut e, chain + 0x30, [0.0, 0.0, 1.0, 0.0]);
        // Coefficients S (at +0x40): S0 = 0, S1 = 4, S2 = 0.25.
        e.mem.set_f32(chain + 0x44, 4.0);
        e.mem.set_f32(chain + 0x48, 0.25);
        let solutions = chain + 0x14 + 0x3c;
        let pairs = chain + 0x14 + 0x40;
        let bodies = e.mem.alloc(0x200);
        e.mem.set_u32(pairs, 0);
        e.mem.set_u32(pairs + 4, 0x100);
        let (body0, body1) = (bodies, bodies + 0x100);
        for (body, base) in [(body0, 1.0f32), (body1, 2.0)] {
            put(&mut e, body + 0x10, [base, 0.0, 0.0, 0.0]);
            put(&mut e, body + 0x20, [0.0, base, 0.0, 0.0]);
            put(&mut e, body + 0x30, [1.0, 1.0, 1.0, 2.0]);
            put(&mut e, body + 0x40, [0.0; 4]);
            put(&mut e, body + 0x50, [0.0; 4]);
        }
        let info = e.mem.alloc(0x80);
        e.mem.set_f32(info + 0x40, 0.5);
        let impulses = e.mem.alloc(0x10);
        e.mem.set_f32(impulses, 1.0);
        e.call(0x00d893a0, &args![info, 0u32, bodies, chain, impulses]);
        // Forward pass: the Jacobian terms cancel (the x and y sums are -3 and 3), so the right
        // side is a * row_a.w = 1 and T[0] = 1 / S1.
        assert_eq!(e.mem.f32(solutions), 0.25);
        // Backward pass: lambda = T - S2 * 0 = 0.25; the impulse vectors are v30 * lambda.
        assert_eq!(e.mem.f32(impulses), 1.25);
        assert_eq!(load_vec(&e, body0 + 0x10), [1.5, 0.0, 0.0, 0.25]);
        assert_eq!(load_vec(&e, body1 + 0x10), [1.5, 0.0, 0.0, -0.25]);
        assert_eq!(load_vec(&e, body0 + 0x20), [0.0, 1.25, 0.0, 0.0]);
        assert_eq!(load_vec(&e, body1 + 0x20), [0.0, 2.0, 0.25, 0.0]);
    }

    #[test]
    fn solver_elements_are_cleared_by_type() {
        let mut e = engine();
        let info = e.mem.alloc(0x40);
        put(&mut e, info + 0x10, [1.0, 2.0, 3.0, 4.0]);
        let block = e.mem.alloc(0x400);
        // Types 0, 0, 1, 2, 3 in a row; the last stops the walk before the sixth.
        for (index, kind) in [0u8, 0, 1, 2, 3, 0].into_iter().enumerate() {
            let element = block + 0x80 * index as u32;
            e.mem.set_u8(element, kind);
            e.mem.set_f32(element + 8, 2.0);
            put(&mut e, element + 0x10, [1.0; 4]);
            put(&mut e, element + 0x40, [9.0; 4]);
            put(&mut e, element + 0x50, [9.0; 4]);
        }
        e.call(0x00d89ed0, &args![info, block, 0u32]);
        for index in 0..4u32 {
            let element = block + 0x80 * index;
            assert_eq!(load_vec(&e, element + 0x40), [0.0; 4]);
            assert_eq!(load_vec(&e, element + 0x50), [0.0; 4]);
        }
        assert_eq!(load_vec(&e, block + 0x10), [3.0, 5.0, 7.0, 9.0]);
        assert_eq!(load_vec(&e, block + 0x90), [3.0, 5.0, 7.0, 9.0]);
        assert_eq!(load_vec(&e, block + 0x110), [1.0; 4]);
        // The type 3 element and the one after it are untouched.
        assert_eq!(load_vec(&e, block + 0x200 + 0x40), [9.0; 4]);
        assert_eq!(load_vec(&e, block + 0x280 + 0x40), [9.0; 4]);
    }

    #[test]
    fn solver_elements_stop_at_the_end_address() {
        let mut e = engine();
        let info = e.mem.alloc(0x40);
        let block = e.mem.alloc(0x400);
        for index in 0..3u32 {
            let element = block + 0x80 * index;
            e.mem.set_u8(element, 1);
            put(&mut e, element + 0x40, [9.0; 4]);
        }
        e.call(0x00d89ed0, &args![info, block, block + 0x100]);
        assert_eq!(load_vec(&e, block + 0x40), [0.0; 4]);
        assert_eq!(load_vec(&e, block + 0xc0), [0.0; 4]);
        assert_eq!(load_vec(&e, block + 0x140), [9.0; 4]);
    }

    #[test]
    #[should_panic(expected = "INT3 at 00d89f64")]
    fn unknown_solver_element_types_hit_the_breakpoint() {
        let mut e = engine();
        let info = e.mem.alloc(0x40);
        let block = e.mem.alloc(0x100);
        e.mem.set_u8(block, 4);
        e.call(0x00d89ed0, &args![info, block, 0u32]);
    }

    #[test]
    fn byte_copy_and_table_lookup() {
        let mut e = engine();
        let source = e.mem.alloc(8);
        let destination = e.mem.alloc(8);
        e.mem.set_u8(source, 0xab);
        e.call(0x00d921c0, &args![destination, source]);
        assert_eq!(e.mem.u8(destination), 0xab);
        // The lookup indexes with the sign-extended byte: 1 -> table[1], 0xff -> table[-1].
        e.mem.set_u8(BYTE_TABLE + 1, 7);
        e.mem.set_u8(BYTE_TABLE.wrapping_sub(1), 9);
        e.mem.set_u8(source, 1);
        assert_eq!(e.call(0x00d921d0, &args![source]).u32(), 7);
        e.mem.set_u8(source, 0xff);
        assert_eq!(e.call(0x00d921d0, &args![source]).u32(), 9);
    }

    /// The globals and parameter block `fn_00d89f80` reads: limit 1, step vector (0, -1, 0, 0),
    /// factor 0.5, scale 0.5 * 1.0, three links; limits records for the indices 0 and 1.
    fn integrate_setup(e: &mut Engine) -> u32 {
        e.set_global(ABS_MASK, 0x7fff_ffffu32);
        e.set_global(ABS_MASK + 4, 0x7fff_ffffu32);
        e.set_global(ABS_MASK + 8, 0x7fff_ffffu32);
        e.set_global(ABS_MASK + 12, 0x7fff_ffffu32);
        for lane in 0..4 {
            e.mem.set_f32(LARGE_VELOCITY + 4 * lane, 1.0e6);
        }
        put(e, CONSTANT_ACCELERATION, [0.0, 0.0, -4.0, 0.0]);
        let params = e.mem.alloc(0x130);
        e.mem.set_f32(params, 1.0);
        put(e, params + 0x10, [0.0, -1.0, 0.0, 0.0]);
        e.mem.set_f32(params + 0x30, 0.5);
        e.mem.set_f32(params + 0x40, 0.5);
        e.mem.set_u32(params + 0x114, 3);
        e.mem.set_f32(params + 0x120, 1.0);
        // Record 0: no weight; record 1: weights 1 and 0.5, factor 0.5, threshold 0.1.
        for (index, value) in [1.0f32, 0.5, 0.5, 0.1].into_iter().enumerate() {
            e.mem.set_f32(params + 0x80 + 4 * index as u32, value);
        }
        params
    }

    #[allow(clippy::too_many_arguments)]
    fn solver_element(
        e: &mut Engine,
        element: u32,
        kind: u8,
        record: u32,
        step: f32,
        linear: Vec4,
        angular: Vec4,
        position: Vec4,
    ) {
        e.mem.set_u8(element, kind);
        e.mem.set_u32(element + 4, record);
        e.mem.set_f32(element + 8, step);
        put(e, element + 0x10, linear);
        put(e, element + 0x20, angular);
        put(e, element + 0x40, position);
        put(e, element + 0x50, [0.0; 4]);
    }

    #[test]
    fn solver_integration_limits_and_advances_each_element_type() {
        let mut e = engine();
        let params = integrate_setup(&mut e);
        let block = e.mem.alloc(0x400);
        let zero = [0.0; 4];
        // Type 2 with a velocity above the stop threshold but below the limit: scaled by 0.5.
        solver_element(&mut e, block, 2, 1, 0.5, [0.5, 0.0, 0.0, 0.0], zero, zero);
        // Type 1: only the integration, with the constant vector.
        solver_element(
            &mut e,
            block + 0x80,
            1,
            0,
            1.0,
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            zero,
        );
        // A run of two type 0 elements; the first has a velocity that is far too large.
        solver_element(
            &mut e,
            block + 0x100,
            0,
            0,
            2.0,
            [3.0e6, 0.0, 0.0, 0.0],
            zero,
            [1.0, 0.0, 0.0, 0.0],
        );
        solver_element(&mut e, block + 0x180, 0, 0, 1.0, zero, zero, zero);
        // Type 3 stops the walk.
        solver_element(&mut e, block + 0x200, 3, 0, 1.0, [5.0; 4], zero, zero);
        solver_element(&mut e, block + 0x280, 1, 0, 1.0, [5.0; 4], zero, zero);
        let flag = e.mem.alloc(8);
        e.mem.set_u8(flag, 9);
        let result = e.call(0x00d89f80, &args![params, 0i32, block, 0u32, flag]);
        assert_eq!(result.u32(), 1);
        assert_eq!(e.mem.u8(flag), 0);
        assert_eq!(load_vec(&e, block + 0x10), [0.375, 0.0, -2.0, 0.0]);
        assert_eq!(load_vec(&e, block + 0x20), zero);
        assert_eq!(load_vec(&e, block + 0x40), [0.125, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, block + 0x80 + 0x10), [1.5, 0.0, -4.0, 0.0]);
        assert_eq!(load_vec(&e, block + 0x80 + 0x20), [0.0, 3.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, block + 0x100 + 0x10), [-0.5, -2.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, block + 0x100 + 0x20), zero);
        assert_eq!(load_vec(&e, block + 0x180 + 0x10), [0.0, -1.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, block + 0x280 + 0x10), [5.0; 4]);
    }

    #[test]
    fn solver_integration_damps_on_the_last_link() {
        let mut e = engine();
        let params = integrate_setup(&mut e);
        let block = e.mem.alloc(0x200);
        let zero = [0.0; 4];
        solver_element(
            &mut e,
            block,
            1,
            0,
            1.0,
            [1.0, 0.0, 0.0, 0.0],
            zero,
            [2.0, 0.0, 0.0, 0.0],
        );
        e.mem.set_u8(block + 0x80, 3);
        let flag = e.mem.alloc(8);
        e.call(0x00d89f80, &args![params, 2i32, block, 0u32, flag]);
        assert_eq!(e.mem.u8(flag), 1);
        // Positions: (2 + 0.5 * -1) = 1.5, then scaled by the damping 0.5; the velocities are
        // the differences (-1, 0).
        assert_eq!(load_vec(&e, block + 0x40), [0.75, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, block + 0x50), zero);
        assert_eq!(load_vec(&e, block + 0x10), [-1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, block + 0x20), zero);
    }

    #[test]
    fn solver_integration_stops_a_type_zero_run_at_the_end_address() {
        let mut e = engine();
        let params = integrate_setup(&mut e);
        let block = e.mem.alloc(0x400);
        let zero = [0.0; 4];
        for index in 0..3u32 {
            solver_element(&mut e, block + 0x80 * index, 0, 0, 1.0, zero, zero, zero);
        }
        let flag = e.mem.alloc(8);
        e.call(0x00d89f80, &args![params, 0i32, block, block + 0x100, flag]);
        assert_eq!(load_vec(&e, block + 0x10), [0.0, -1.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, block + 0x90), [0.0, -1.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, block + 0x110), zero);
    }

    #[test]
    #[should_panic(expected = "00d8a008")]
    fn solver_integration_cannot_leave_an_unknown_type() {
        let mut e = engine();
        let params = integrate_setup(&mut e);
        let block = e.mem.alloc(0x100);
        e.mem.set_u8(block, 7);
        let flag = e.mem.alloc(8);
        e.call(0x00d89f80, &args![params, 0i32, block, 0u32, flag]);
    }

    /// Small integers, so that every sum and product in the chain tests is exact.
    fn pattern(seed: u32, index: u32) -> f32 {
        ((seed * 5 + index * 3) % 4) as f32 - 1.0
    }

    fn put_pattern(e: &mut Engine, at: u32, seed: u32) {
        put(
            e,
            at,
            [
                pattern(seed, 0),
                pattern(seed, 1),
                pattern(seed, 2),
                pattern(seed, 3),
            ],
        );
    }

    type Wide = [f64; 4];

    fn wide(e: &Engine, at: u32) -> Wide {
        load_vec(e, at).map(f64::from)
    }

    fn wide_add(a: Wide, b: Wide) -> Wide {
        std::array::from_fn(|lane| a[lane] + b[lane])
    }

    fn wide_sub(a: Wide, b: Wide) -> Wide {
        std::array::from_fn(|lane| a[lane] - b[lane])
    }

    fn wide_mul(a: Wide, b: Wide) -> Wide {
        std::array::from_fn(|lane| a[lane] * b[lane])
    }

    fn wide_scale(a: Wide, factor: f64) -> Wide {
        a.map(|lane| lane * factor)
    }

    /// `x * rows[0] + y * rows[1] + z * rows[2]` for the first three lanes of `weights`.
    fn wide_combine(weights: Wide, rows: [Wide; 3]) -> Wide {
        wide_add(
            wide_add(
                wide_scale(rows[0], weights[0]),
                wide_scale(rows[1], weights[1]),
            ),
            wide_scale(rows[2], weights[2]),
        )
    }

    /// The vectors of one body, as the chain solver sees them.
    #[derive(Clone, Copy)]
    struct WideBody {
        linear: Wide,
        angular: Wide,
        moment: Wide,
        position: Wide,
        rotation: Wide,
    }

    fn wide_body(e: &Engine, at: u32) -> WideBody {
        WideBody {
            linear: wide(e, at + 0x10),
            angular: wide(e, at + 0x20),
            moment: wide(e, at + 0x30),
            position: wide(e, at + 0x40),
            rotation: wide(e, at + 0x50),
        }
    }

    #[test]
    fn ball_and_socket_chain_solves_two_links() {
        let mut e = engine();
        let solver_info = e.mem.alloc(0x80);
        e.mem.set_f32(solver_info + 0x40, 2.0);
        let chain = e.mem.alloc(0x300);
        e.mem.set_u16(chain + 2, 2);
        e.mem.set_f32(chain + 8, 1.0);
        e.mem.set_f32(chain + 0xc, 3.0);
        let bodies = e.mem.alloc(0x180);
        let offsets = [0x0u32, 0x80, 0x100];
        let pairs = chain + 0x20 + 0x130 * 2;
        for (index, offset) in offsets.iter().enumerate() {
            e.mem.set_u32(pairs + 4 * index as u32, *offset);
            for (field, at) in [0x10u32, 0x20, 0x30, 0x40, 0x50].into_iter().enumerate() {
                put_pattern(
                    &mut e,
                    bodies + offset + at,
                    7 * index as u32 + field as u32,
                );
            }
        }
        for link in 0..2u32 {
            for k in 0..9u32 {
                put_pattern(&mut e, chain + 0x10 + 0x90 * link + 0x10 * k, 3 * link + k);
                put_pattern(
                    &mut e,
                    chain + 0x10 + 0x90 * 2 + 0x90 * link + 0x10 * k,
                    11 + 5 * link + k,
                );
            }
        }
        let results = e.mem.alloc(0x20);
        for slot in 0..6u32 {
            e.mem.set_f32(results + 4 * slot, slot as f32 + 1.0);
        }

        // The same solve in plain wide arithmetic.
        let links = |e: &Engine, link: u32| -> [Wide; 9] {
            std::array::from_fn(|k| wide(e, chain + 0x10 + 0x90 * link + 0x10 * k as u32))
        };
        let factors = |e: &Engine, link: u32| -> [Wide; 9] {
            std::array::from_fn(|k| {
                wide(e, chain + 0x10 + 0x90 * 2 + 0x90 * link + 0x10 * k as u32)
            })
        };
        let mut model: Vec<WideBody> = offsets
            .iter()
            .map(|offset| wide_body(&e, bodies + offset))
            .collect();
        let mut rights = vec![[0.0f64; 4]; 3];
        for link in 0..2usize {
            let v = links(&e, link as u32);
            let w = factors(&e, link as u32);
            let (a, b) = (model[link], model[link + 1]);
            let offset = wide_sub(a.position, b.position);
            let relative = wide_sub(
                wide_sub(a.linear, a.position),
                wide_sub(b.linear, b.position),
            );
            let angular_a = wide_sub(a.angular, a.rotation);
            let angular_b = wide_sub(b.angular, b.rotation);
            let mut residual = [0.0f64; 4];
            for (k, slot) in residual.iter_mut().take(3).enumerate() {
                let p = wide_add(
                    wide_add(
                        wide_mul(v[3 * k + 1], a.rotation),
                        wide_mul(v[3 * k + 2], b.rotation),
                    ),
                    wide_mul(offset, v[3 * k]),
                );
                let q = wide_add(
                    wide_add(
                        wide_mul(v[3 * k + 1], angular_a),
                        wide_mul(v[3 * k + 2], angular_b),
                    ),
                    wide_mul(v[3 * k], relative),
                );
                let x = wide_add(wide_scale(p, 2.0), wide_scale(q, 3.0));
                *slot = v[3 * k][3] - (x[0] + x[1] + x[2]);
            }
            let carried = wide_combine(rights[link], [w[0], w[1], w[2]]);
            let rest = wide_sub(residual, carried);
            rights[link + 1] = wide_combine(rest, [w[3], w[4], w[5]]);
        }
        let mut expected_results = [1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut solution = [0.0f64; 4];
        for link in (0..2usize).rev() {
            let v = links(&e, link as u32);
            let w = factors(&e, link as u32);
            let correction = wide_combine(solution, [w[6], w[7], w[8]]);
            solution = wide_sub(rights[link + 1], correction);
            for component in 0..3usize {
                let value = solution[component];
                let moment_a = wide_scale(model[link].moment, value);
                let moment_b = wide_scale(model[link + 1].moment, value);
                model[link].linear = wide_add(
                    wide_scale(v[3 * component], moment_a[3]),
                    model[link].linear,
                );
                model[link + 1].linear = wide_sub(
                    model[link + 1].linear,
                    wide_scale(v[3 * component], moment_b[3]),
                );
                let angular_a = wide_add(
                    wide_mul(v[3 * component + 1], moment_a),
                    model[link].angular,
                );
                let angular_b = wide_add(
                    wide_mul(v[3 * component + 2], moment_b),
                    model[link + 1].angular,
                );
                model[link + 1].angular = angular_b;
                model[link].angular = angular_a;
                expected_results[3 * link + component] += value;
            }
        }

        e.call(
            0x00d8_9690,
            &args![solver_info, 0xdeadu32, bodies, chain, results],
        );
        for (index, offset) in offsets.iter().enumerate() {
            let at = bodies + offset;
            assert_eq!(wide(&e, at + 0x10), model[index].linear, "linear {index}");
            assert_eq!(wide(&e, at + 0x20), model[index].angular, "angular {index}");
            // The solver leaves the other vectors alone.
            assert_eq!(wide(&e, at + 0x30), model[index].moment);
            assert_eq!(wide(&e, at + 0x40), model[index].position);
            assert_eq!(wide(&e, at + 0x50), model[index].rotation);
        }
        for (slot, wanted) in expected_results.iter().enumerate() {
            assert_eq!(f64::from(e.mem.f32(results + 4 * slot as u32)), *wanted);
        }
        // The right-hand sides: the first is zero and the last link's is the one used.
        assert_eq!(wide(&e, chain + 0x10 + 0x120 * 2), [0.0; 4]);
        assert_eq!(wide(&e, chain + 0x10 + 0x120 * 2 + 0x20), rights[2]);
        // An empty chain only clears its first right-hand side.
        let empty = e.mem.alloc(0x100);
        put(&mut e, empty + 0x10, [9.0; 4]);
        e.call(
            0x00d8_9690,
            &args![solver_info, 0u32, bodies, empty, results],
        );
        assert_eq!(load_vec(&e, empty + 0x10), [0.0; 4]);
    }

    type WordLog = std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>>;

    #[test]
    fn powered_chain_runs_the_search_loops_and_the_backward_pass() {
        use std::cell::RefCell;
        use std::rc::Rc;
        let mut e = engine();
        e.call_log = Some(Vec::new());
        let chain = e.mem.alloc(0xc00);
        e.mem.set_u16(chain + 4, 2);
        e.mem.set_f32(chain + 0x18, 1.5);
        // A signalling NaN: the copy through the x87 stack quiets it.
        e.mem.set_u32(chain + 0x1c, 0x7f80_0001);
        let links = chain + 0x20;
        let seconds = chain + 0x140;
        let matrices = chain + 0x200;
        let vectors = chain + 0x980;
        let rows = chain + 0x9e0;
        let pairs = chain + 0xa20;
        let flags = chain + 0xa2c;
        let step = e.mem.alloc(0x10);
        e.mem.set_f32(step + 4, 2.0);
        let bodies = e.mem.alloc(0x180);
        let offsets = [0x0u32, 0x80, 0x100];
        for (index, offset) in offsets.iter().enumerate() {
            e.mem.set_u32(pairs + 4 * index as u32, *offset);
            for (field, at) in [0x10u32, 0x20, 0x30, 0x40, 0x50].into_iter().enumerate() {
                put_pattern(
                    &mut e,
                    bodies + offset + at,
                    7 * index as u32 + field as u32,
                );
            }
        }
        for link in 0..2u32 {
            for k in 0..9u32 {
                put_pattern(&mut e, links + 0x90 * link + 0x10 * k, 3 * link + k);
            }
            for k in 0..6u32 {
                put_pattern(&mut e, seconds + 0x60 * link + 0x10 * k, 20 + 4 * link + k);
            }
            for k in 0..12u32 {
                put_pattern(
                    &mut e,
                    matrices + 0x3c0 * link + 0x180 + 0x10 * k,
                    30 + 13 * link + k,
                );
            }
            put_pattern(&mut e, vectors + 0x20 + 0x20 * link, 50 + link);
            put_pattern(&mut e, vectors + 0x30 + 0x20 * link, 52 + link);
        }
        // The flag records: three two-bit modes in the first byte, three source floats.
        e.mem.set_u8(flags, 0x24);
        e.mem.set_u8(flags + 0x40, 0x0b);
        for (link, values) in [[3.0f32, 5.0, 7.0], [1.0, 2.0, 3.0]].iter().enumerate() {
            for (value, at) in values.iter().zip([0xc, 0x20, 0x34]) {
                e.mem.set_f32(flags + 0x40 * link as u32 + at, *value);
            }
        }
        for (link, values) in [[0.5f32, 1.5, 2.5], [10.0, 20.0, 30.0]].iter().enumerate() {
            for (value, at) in values.iter().zip([0x1c, 0x3c, 0x5c]) {
                e.mem.set_f32(seconds + 0x60 * link as u32 + at, *value);
            }
        }
        e.mem.set_f32(rows + 0x10, 7.0);
        e.mem.set_f32(rows + 0x10 + 8, 9.0);
        e.mem.set_f32(rows + 0x10 + 4 * 9, 9.0);
        let results = e.mem.alloc(0x40);
        for slot in 0..12u32 {
            e.mem.set_f32(results + 4 * slot, slot as f32 + 1.0);
        }

        // The doubles: the first search finds link 1 (with 5 as its second value), the second
        // finds link 0 (with 9); both then find nothing.
        let context: WordLog = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&context);
        e.register_double(FN_00DF0D30, move |e, a| {
            kept.borrow_mut()
                .push((0..12).map(|word| e.mem.u32(a[0] + 4 * word)).collect());
            Ret::default()
        });
        let outputs: WordLog = Rc::new(RefCell::new(Vec::new()));
        for (routine, found, second) in [(FN_00DF1430, 1u32, 5u32), (FN_00DF1260, 0, 9)] {
            let kept = Rc::clone(&outputs);
            let mut calls = 0;
            e.register_double(routine, move |e, a| {
                kept.borrow_mut()
                    .push(vec![e.mem.u32(a[1]), e.mem.u32(a[2]), e.mem.u32(a[3])]);
                if calls == 0 {
                    e.mem.set_u32(a[1], found);
                    e.mem.set_u32(a[2], second);
                }
                calls += 1;
                Ret::default()
            });
        }
        for routine in [FN_00DF0810, FN_00DF11A0, FN_00DF2040, FN_00DF1060] {
            e.register_double(routine, |_, _| Ret::default());
        }
        let model_start: Vec<WideBody> = offsets
            .iter()
            .map(|offset| wide_body(&e, bodies + offset))
            .collect();
        e.call(
            0x00d8_a3a0,
            &args![0x5000_0000u32, step, bodies, chain, results],
        );

        // The context block handed to the routines.
        assert_eq!(
            context.borrow()[0],
            [
                0x5000_0000,
                2,
                1.5f32.to_bits(),
                0x7fc0_0001,
                links,
                seconds,
                pairs,
                bodies,
                matrices,
                flags,
                vectors,
                rows
            ]
        );
        // `source * step[+4]` was added to the three `w` lanes of each second record.
        let sums = [6.5f32, 11.5, 16.5, 12.0, 24.0, 36.0];
        for (index, wanted) in sums.iter().enumerate() {
            let at = seconds + 0x60 * (index as u32 / 3) + [0x1c, 0x3c, 0x5c][index % 3];
            assert_eq!(e.mem.f32(at), *wanted);
        }
        // Mode 2 zeroes the row slot, the other modes leave it alone.
        assert_eq!(e.mem.f32(rows + 0x10), 7.0);
        assert_eq!(e.mem.f32(rows + 0x10 + 8), 0.0);
        assert_eq!(e.mem.f32(rows + 0x10 + 4 * 9), 0.0);
        // The order of the calls and the arguments the loops pass.
        let context_pointer = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|(callee, _)| *callee == FN_00DF0D30)
            .unwrap()
            .1[0];
        let order: Vec<(u32, Vec<u32>)> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(callee, _)| {
                [
                    FN_00DF0D30,
                    FN_00DF0810,
                    FN_00DF11A0,
                    FN_00DF1430,
                    FN_00DF1260,
                    FN_00DF2040,
                    FN_00DF1060,
                ]
                .contains(callee)
            })
            .cloned()
            .collect();
        let found_slot = order[4].1[1];
        let second_slot = order[4].1[2];
        let flag_slot = order[4].1[3];
        assert_eq!(found_slot, flag_slot - 0x20);
        assert_eq!(second_slot, found_slot + 4);
        assert_eq!(context_pointer, found_slot + 0x44);
        let refresh = (FN_00DF11A0, vec![context_pointer, vectors]);
        let search = |routine: u32| {
            (
                routine,
                vec![context_pointer, found_slot, second_slot, flag_slot, flags],
            )
        };
        let expected = vec![
            (FN_00DF0D30, vec![context_pointer, rows]),
            (FN_00DF0810, vec![0, 1, flags, rows]),
            (FN_00DF0810, vec![1, 0, flags, rows]),
            refresh.clone(),
            search(FN_00DF1430),
            (FN_00DF2040, vec![1, 2, flags, matrices]),
            (FN_00DF1060, vec![1, 5, context_pointer, rows]),
            refresh.clone(),
            search(FN_00DF1430),
            refresh.clone(),
            search(FN_00DF1260),
            (FN_00DF2040, vec![0, 2, flags, matrices]),
            (FN_00DF0810, vec![0, 9, flags, rows]),
            refresh,
            search(FN_00DF1260),
        ];
        assert_eq!(order, expected);
        // Each search starts with its outputs reset to -1, -1 and 0.
        assert_eq!(*outputs.borrow(), vec![vec![u32::MAX, u32::MAX, 0]; 4]);

        // The backward pass in plain wide arithmetic.
        let mut model = model_start;
        let mut previous = [[0.0f64; 4]; 2];
        let mut expected_results: Vec<f64> = (1..=12).map(f64::from).collect();
        for link in (0..2usize).rev() {
            let link_u = link as u32;
            let m: Vec<Wide> = (0..12)
                .map(|k| wide(&e, matrices + 0x3c0 * link_u + 0x180 + 0x10 * k))
                .collect();
            let [p, q] = previous;
            let from_p: Wide = std::array::from_fn(|lane| {
                p[0] * m[0][lane] + p[1] * m[1][lane] + p[2] * m[2][lane]
            });
            let from_q: Wide = std::array::from_fn(|lane| {
                q[0] * m[3][lane] + q[1] * m[4][lane] + q[2] * m[5][lane]
            });
            let fourth: Wide = std::array::from_fn(|lane| {
                q[0] * m[9][lane]
                    + q[1] * m[10][lane]
                    + q[2] * m[11][lane]
                    + (p[0] * m[6][lane] + p[1] * m[7][lane] + p[2] * m[8][lane])
            });
            let pair = vectors + 0x20 + 0x20 * link_u;
            let first = wide_sub(wide(&e, pair), wide_add(from_q, from_p));
            let last = wide_sub(wide(&e, pair + 0x10), fourth);
            let v: Vec<Wide> = (0..9)
                .map(|k| wide(&e, links + 0x90 * link_u + 0x10 * k))
                .collect();
            let r: Vec<Wide> = (0..6)
                .map(|k| wide(&e, seconds + 0x60 * link_u + 0x10 * k))
                .collect();
            for component in 0..3usize {
                let value = first[component];
                let moment_a = wide_scale(model[link].moment, value);
                let moment_b = wide_scale(model[link + 1].moment, value);
                model[link].linear = wide_add(
                    wide_scale(v[3 * component], moment_a[3]),
                    model[link].linear,
                );
                model[link + 1].linear = wide_sub(
                    model[link + 1].linear,
                    wide_scale(v[3 * component], moment_b[3]),
                );
                model[link].angular = wide_add(
                    wide_mul(v[3 * component + 1], moment_a),
                    model[link].angular,
                );
                model[link + 1].angular = wide_add(
                    wide_mul(v[3 * component + 2], moment_b),
                    model[link + 1].angular,
                );
                expected_results[6 * link + component] += value;
            }
            for component in 0..3usize {
                let value = last[component];
                let moment_a = wide_scale(model[link].moment, value);
                model[link].angular =
                    wide_add(wide_mul(moment_a, r[2 * component]), model[link].angular);
                let moment_b = wide_scale(model[link + 1].moment, value);
                model[link + 1].angular = wide_add(
                    wide_mul(moment_b, r[2 * component + 1]),
                    model[link + 1].angular,
                );
                expected_results[6 * link + 3 + component] += value;
            }
            previous = [first, last];
        }
        for (index, offset) in offsets.iter().enumerate() {
            let at = bodies + offset;
            assert_eq!(wide(&e, at + 0x10), model[index].linear, "linear {index}");
            assert_eq!(wide(&e, at + 0x20), model[index].angular, "angular {index}");
            assert_eq!(wide(&e, at + 0x30), model[index].moment);
        }
        for (slot, wanted) in expected_results.iter().enumerate() {
            assert_eq!(f64::from(e.mem.f32(results + 4 * slot as u32)), *wanted);
        }
        // Without links only the context routine, the two searches and their refreshes run.
        let empty = e.mem.alloc(0x100);
        e.call_log = Some(Vec::new());
        e.call(0x00d8_a3a0, &args![0u32, step, 0u32, empty, results]);
        let sequence: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(callee, _)| *callee)
            .filter(|callee| *callee != 0x00d8_a3a0)
            .collect();
        assert_eq!(
            sequence,
            [
                FN_00DF0D30,
                FN_00DF11A0,
                FN_00DF1430,
                FN_00DF11A0,
                FN_00DF1260
            ]
        );
    }

    /// The byte table `011b9434` the row types index: the number of floats the accumulator
    /// pointer moves after a row of that type (`255` for the types that never reach it).
    const ADVANCE_BY_TYPE: [u8; 32] = [
        255, 255, 255, 255, 255, 1, 1, 1, 1, 2, 3, 1, 1, 1, 2, 3, 1, 1, 1, 1, 2, 2, 3, 4, 3, 0, 0,
        0, 255, 255, 255, 255,
    ];

    /// One step of a schema built by [`schema`].
    #[derive(Clone, Copy)]
    enum Step {
        /// Type 1: select the bodies with these two indices.
        Bodies(u16, u16),
        /// Type 2 with this many bytes.
        Skip(u32),
        /// Type 3.
        SkipSixteen,
        /// A Jacobian row of this type (5 to 27).
        Row(u8),
        /// A chain record (type 29, 30 or 31) with this many links.
        Chain(u8, u16),
    }

    /// The buffers a solver call works on.
    struct Fixture {
        info: u32,
        schema: u32,
        bodies: u32,
        accum: u32,
    }

    /// Change positions are byte offsets into the info block (`0`), the four bodies (`0x200`)
    /// and the accumulators (`0x400`); the value is the new bit pattern.
    type Change = (u32, u32);

    const BODIES_AT: u32 = 0x200;
    const ACCUM_AT: u32 = 0x400;

    /// An odd multiple of 1/16 between -0.8125 and 0.6875 (never zero), by position and seed.
    fn sixteenths(index: u32, seed: u32) -> f32 {
        (2.0 * ((index * 5 + seed * 3) % 13) as f32 - 13.0) / 16.0
    }

    /// The size of a Jacobian row record by type.
    fn row_size(kind: u8) -> u32 {
        match kind {
            5 | 6 | 13 | 14 | 16..=19 => 0x30,
            7 | 8 | 9 | 11 | 15 => 0x40,
            10 | 24 => 0x50,
            12 | 25 | 26 => 0x20,
            20..=22 => 0x70,
            23 => 0x90,
            27 => 0x60,
            _ => panic!("type {kind} is not a row"),
        }
    }

    /// The size of a chain record by type and link count (the exe's solvers read it at `+4`,
    /// `+0x10` for the powered chain).
    fn chain_size(kind: u8, links: u16) -> u32 {
        let links = u32::from(links);
        let (fixed, per_link) = match kind {
            29 => (0x14, 0x40),
            30 => (0x20, 0x130),
            _ => (0x44, 0x534),
        };
        (fixed + per_link * links + 4 * (links + 1)).div_ceil(16) * 16
    }

    /// Fills `count` floats at `at` with the pattern of `seed`.
    fn fill(e: &mut Engine, at: u32, count: u32, seed: u32) {
        for index in 0..count {
            e.mem.set_f32(at + 4 * index, sixteenths(index, seed));
        }
    }

    /// The info block, four bodies (velocities and positive inverse masses) and accumulators of
    /// a solver call, and a schema built from `steps`.
    fn fixture(e: &mut Engine, steps: &[Step]) -> Fixture {
        let info = e.mem.alloc(0x200);
        fill(e, info, 0x80, 1);
        e.mem.set_f32(info + 8, 0.5);
        e.mem.set_f32(info + 0x30, 0.5);
        e.mem.set_f32(info + 0x40, 0.25);
        let bodies = e.mem.alloc(0x200);
        for body in 0..4 {
            let at = bodies + 0x80 * body;
            fill(e, at + 0x10, 0x14, 2 + body);
            for lane in 0..4 {
                e.mem
                    .set_f32(at + 0x30 + 4 * lane, 0.25 * (1 + (body + lane) % 3) as f32);
            }
        }
        let accum = e.mem.alloc(0x80);
        fill(e, accum, 0x20, 3);
        let schema = schema(e, steps);
        Fixture {
            info,
            schema,
            bodies,
            accum,
        }
    }

    /// A schema that selects bodies 0 and 1, runs `steps` and ends.
    fn schema(e: &mut Engine, steps: &[Step]) -> u32 {
        let size: u32 = 0x40
            + steps
                .iter()
                .map(|step| match *step {
                    Step::Bodies(..) | Step::SkipSixteen => 0x10,
                    Step::Skip(bytes) => bytes,
                    Step::Row(kind) => row_size(kind),
                    Step::Chain(kind, links) => chain_size(kind, links),
                })
                .sum::<u32>();
        let schema = e.mem.alloc(size);
        e.mem.set_u8(schema, 1);
        e.mem.set_u16(schema + 6, 1);
        let mut at = schema + 0x10;
        for (position, step) in steps.iter().enumerate() {
            let seed = 7 + position as u32;
            match *step {
                Step::Bodies(first, second) => {
                    e.mem.set_u8(at, 1);
                    e.mem.set_u16(at + 4, first);
                    e.mem.set_u16(at + 6, second);
                    at += 0x10;
                }
                Step::Skip(bytes) => {
                    e.mem.set_u8(at, 2);
                    e.mem.set_u32(at + 4, bytes);
                    at += bytes;
                }
                Step::SkipSixteen => {
                    e.mem.set_u8(at, 3);
                    at += 0x10;
                }
                Step::Row(kind) => {
                    let bytes = row_size(kind);
                    fill(e, at, bytes / 4, u32::from(kind) + seed);
                    e.mem.set_u8(at, kind);
                    at += bytes;
                }
                Step::Chain(kind, links) => {
                    let bytes = chain_size(kind, links);
                    fill(e, at, bytes / 4, u32::from(kind) + seed);
                    e.mem.set_u8(at, kind);
                    let (count_at, size_at, pairs) = match kind {
                        29 => (2, 4, 0x14 + 0x40 * u32::from(links)),
                        30 => (2, 4, 0x20 + 0x130 * u32::from(links)),
                        _ => (4, 0x10, 0x40 + 0x4f0 * u32::from(links)),
                    };
                    e.mem.set_u16(at + count_at, links);
                    e.mem.set_u32(at + size_at, bytes);
                    for link in 0..=u32::from(links) {
                        e.mem.set_u32(at + pairs + 4 * link, 0x80 * (link % 2));
                    }
                    at += bytes;
                }
            }
        }
        schema
    }

    /// Every word of the info block, the bodies and the accumulators.
    fn snapshot(e: &Engine, fx: &Fixture) -> Vec<u32> {
        let mut words = Vec::new();
        for (base, count) in [(fx.info, 0x80), (fx.bodies, 0x80), (fx.accum, 0x20)] {
            words.extend((0..count).map(|index| e.mem.u32(base + 4 * index)));
        }
        words
    }

    /// The words that differ between two snapshots, as `(offset, new bits)`.
    fn changes(before: &[u32], after: &[u32]) -> Vec<Change> {
        before
            .iter()
            .zip(after)
            .enumerate()
            .filter(|(_, (old, new))| old != new)
            .map(|(index, (_, new))| (4 * index as u32, *new))
            .collect()
    }

    /// The schemas the golden tests run: one row of each type 5 to 27 (scenarios 0 to 22), then
    /// row sequences, body switches and skips, and chains.
    const SCENARIOS: [&[Step]; 37] = [
        &[Step::Row(5)],
        &[Step::Row(6)],
        &[Step::Row(7)],
        &[Step::Row(8)],
        &[Step::Row(9)],
        &[Step::Row(10)],
        &[Step::Row(11)],
        &[Step::Row(12)],
        &[Step::Row(13)],
        &[Step::Row(14)],
        &[Step::Row(15)],
        &[Step::Row(16)],
        &[Step::Row(17)],
        &[Step::Row(18)],
        &[Step::Row(19)],
        &[Step::Row(20)],
        &[Step::Row(21)],
        &[Step::Row(22)],
        &[Step::Row(23)],
        &[Step::Row(24)],
        &[Step::Row(25)],
        &[Step::Row(26)],
        &[Step::Row(27)],
        &[Step::Row(20), Step::Row(23)],
        &[Step::Row(22), Step::Row(23)],
        &[Step::Row(20), Step::Row(20), Step::Row(21), Step::Row(5)],
        &[Step::Row(5), Step::Row(5), Step::Row(6), Step::Row(6)],
        &[
            Step::Bodies(2, 3),
            Step::Skip(0x20),
            Step::SkipSixteen,
            Step::Row(5),
            Step::Row(12),
        ],
        &[Step::Chain(29, 1)],
        &[Step::Chain(29, 2), Step::Row(5)],
        &[Step::Chain(30, 1)],
        &[Step::Chain(30, 2), Step::Row(5)],
        &[Step::Row(7), Step::Row(8), Step::Row(7)],
        &[Step::Row(10), Step::Row(10), Step::Row(11)],
        &[Step::Row(9), Step::Row(9)],
        &[Step::Row(16), Step::Row(17), Step::Row(18), Step::Row(19)],
        &[Step::Row(24), Step::Row(25), Step::Row(26), Step::Row(27)],
    ];

    const GOLDEN_FIRST: [(usize, &[Change]); 37] = [
        (
            0,
            &[
                (0x210, 0xbd9e4c01),
                (0x214, 0x3f14de80),
                (0x218, 0xbee11500),
                (0x21c, 0x3e283200),
                (0x220, 0xbf4d4b80),
                (0x224, 0xbe55a400),
                (0x228, 0x3f01da80),
                (0x22c, 0xbf0f7580),
                (0x290, 0x3eaf2600),
                (0x294, 0xbf39bd00),
                (0x298, 0xbd6eb000),
                (0x29c, 0x3f1be700),
                (0x2a0, 0xbef37a00),
                (0x2a4, 0x3e6d7200),
                (0x2a8, 0xbf519f80),
                (0x2ac, 0xbe07bc00),
                (0x400, 0x3e358000),
            ],
        ),
        (
            1,
            &[
                (0x210, 0x3db80008),
                (0x214, 0x3f190000),
                (0x218, 0xbf050000),
                (0x21c, 0x3e960000),
                (0x220, 0xbf530000),
                (0x224, 0xbee40000),
                (0x228, 0x3f1d0000),
                (0x22c, 0xbf1f0000),
                (0x290, 0x3bffff00),
                (0x294, 0xbf420000),
                (0x298, 0x3dd00000),
                (0x29c, 0x3eb40000),
                (0x2a0, 0xbe380000),
                (0x2a4, 0x3e640000),
                (0x2a8, 0xbf6b0000),
                (0x2ac, 0xbcc00000),
                (0x400, 0xbee00000),
            ],
        ),
        (
            2,
            &[
                (0x210, 0xbe06ac9a),
                (0x214, 0x3f17927e),
                (0x218, 0xbf10d022),
                (0x21c, 0x3e21b608),
                (0x220, 0xbf3e54da),
                (0x224, 0xbebadde8),
                (0x228, 0x3eef24fc),
                (0x22c, 0xbee87864),
                (0x290, 0x3ee6ac9a),
                (0x294, 0xbf3f24fc),
                (0x298, 0x3e468110),
                (0x29c, 0x3f1f24fc),
                (0x2a0, 0xbf093da4),
                (0x2a4, 0x3eabb8ec),
                (0x2a8, 0xbf6bc3ce),
                (0x2ac, 0xbe543150),
                (0x400, 0x3f718a80),
            ],
        ),
        (
            3,
            &[
                (0x210, 0xbd96a401),
                (0x214, 0x3f297880),
                (0x218, 0xbf03cf80),
                (0x21c, 0x3e61f600),
                (0x220, 0xbf74ca80),
                (0x224, 0xbe81f600),
                (0x228, 0x3f2b6e80),
                (0x22c, 0xbf297880),
                (0x290, 0x3eab5201),
                (0x294, 0xbf62f100),
                (0x298, 0x3dbcf800),
                (0x29c, 0x3efe0a00),
                (0x2a0, 0xbed4ae00),
                (0x2a4, 0x3f0d6480),
                (0x2a8, 0xbf5e2680),
                (0x2ac, 0xbd9d9800),
                (0x400, 0x3f829000),
            ],
        ),
        (
            4,
            &[
                (0x210, 0xbd1bfffa),
                (0x214, 0x3f024000),
                (0x218, 0xbee28000),
                (0x21c, 0x3e6d0000),
                (0x220, 0xbf58c000),
                (0x224, 0xbe220000),
                (0x228, 0x3e7d0000),
                (0x22c, 0xbf13c000),
                (0x290, 0x3e86ffff),
                (0x294, 0xbf148000),
                (0x298, 0xbd580000),
                (0x29c, 0x3ef30000),
                (0x2a0, 0xbebd0000),
                (0x2a4, 0x3d640000),
                (0x2a8, 0xbf4ec000),
                (0x2ac, 0xbda40000),
                (0x400, 0x3f200000),
                (0x404, 0xbf60bf08),
            ],
        ),
        (
            5,
            &[
                (0x210, 0x3d64001c),
                (0x214, 0x3f024000),
                (0x218, 0xbec48000),
                (0x21c, 0x3d8e0000),
                (0x220, 0xbf52c000),
                (0x224, 0x3bc00000),
                (0x228, 0x3e590000),
                (0x22c, 0xbf07c000),
                (0x290, 0x3d9bffe4),
                (0x294, 0xbf148000),
                (0x298, 0xbe2e0000),
                (0x29c, 0x3f4c8000),
                (0x2a0, 0xbf378000),
                (0x2a4, 0x3dba0000),
                (0x2a8, 0xbf3cc000),
                (0x2ac, 0xbec30000),
                (0x400, 0x3f800000),
                (0x404, 0xc0bbb259),
            ],
        ),
        (
            6,
            &[
                (0x210, 0xbd217ff9),
                (0x214, 0x3f0f5800),
                (0x218, 0xbeee7000),
                (0x21c, 0x3e4d2000),
                (0x220, 0xbf4b6800),
                (0x224, 0xbe4fc000),
                (0x228, 0x3f099800),
                (0x22c, 0xbf0e0800),
                (0x290, 0x3ead2000),
                (0x294, 0xbf3e7000),
                (0x298, 0xbd8a8000),
                (0x29c, 0x3f1bd000),
                (0x2a0, 0xbef26000),
                (0x2a4, 0x3e837000),
                (0x2a8, 0xbf50a800),
                (0x2ac, 0xbe79c000),
                (0x400, 0x3e180000),
            ],
        ),
        (
            7,
            &[
                (0x220, 0xbf501380),
                (0x224, 0xbe3e9400),
                (0x228, 0x3edea100),
                (0x22c, 0xbf0ff980),
                (0x2a0, 0xbedee200),
                (0x2a4, 0x3e3e7a00),
                (0x2a8, 0xbf4fdf80),
                (0x2ac, 0xbe423c00),
                (0x400, 0x3ea34000),
            ],
        ),
        (
            8,
            &[
                (0x220, 0xbf4f232c),
                (0x224, 0xbe5de761),
                (0x228, 0x3edad308),
                (0x22c, 0xbf0dfcbc),
                (0x2a0, 0xbeea59f0),
                (0x2a4, 0x3e437350),
                (0x2a8, 0xbf4cd64c),
                (0x2ac, 0xbe4b8060),
                (0x400, 0x3ec4ce01),
            ],
        ),
        (
            9,
            &[
                (0x220, 0xbf483b01),
                (0x224, 0xbe7057f8),
                (0x228, 0x3eef89fd),
                (0x22c, 0xbf1b38fe),
                (0x2a0, 0xbeea5bfe),
                (0x2a4, 0x3e8441fa),
                (0x2a8, 0xbf57c4ff),
                (0x2ac, 0xbe391801),
                (0x400, 0x3f073ff7),
                (0x404, 0x3f800000),
            ],
        ),
        (
            10,
            &[
                (0x220, 0xbf373ffe),
                (0x224, 0xbe2e0000),
                (0x228, 0x3e4d0000),
                (0x22c, 0xbf004000),
                (0x2a0, 0xbefb0000),
                (0x2a4, 0x3f07c000),
                (0x2a8, 0xbf494000),
                (0x2ac, 0xbe9f0000),
                (0x400, 0xbe800000),
                (0x404, 0xc0a33766),
            ],
        ),
        (
            11,
            &[
                (0x210, 0xbe0b2808),
                (0x214, 0x3f22ca00),
                (0x218, 0xbf195600),
                (0x21c, 0x3e30f800),
                (0x220, 0xbf2e2e00),
                (0x224, 0xbec93802),
                (0x228, 0x3f11d200),
                (0x22c, 0xbf40da00),
                (0x290, 0x3eeb2808),
                (0x294, 0xbf559400),
                (0x298, 0x3e855802),
                (0x29c, 0x3f178400),
                (0x2a0, 0xbf068c00),
                (0x2a4, 0x3efdd402),
                (0x2a8, 0xbf71d200),
                (0x2ac, 0xbe21f000),
                (0x400, 0x3fa04002),
            ],
        ),
        (
            12,
            &[
                (0x210, 0xbd597ffb),
                (0x214, 0x3f2a7800),
                (0x218, 0xbef81000),
                (0x21c, 0x3e702000),
                (0x220, 0xbf6a7800),
                (0x224, 0xbe534000),
                (0x228, 0x3f30f800),
                (0x22c, 0xbf20d800),
                (0x290, 0x3e965fff),
                (0x294, 0xbf64f000),
                (0x298, 0x3d010000),
                (0x29c, 0x3eefe000),
                (0x2a0, 0xbec32000),
                (0x2a4, 0xbe376000),
                (0x2a8, 0xbf573800),
                (0x2ac, 0xbd650000),
                (0x400, 0x3f6a0000),
            ],
        ),
        (
            13,
            &[
                (0x210, 0xbe3e000a),
                (0x214, 0x3f388000),
                (0x218, 0xbee90000),
                (0x21c, 0xbbc00000),
                (0x220, 0xbf398000),
                (0x224, 0xbeba0000),
                (0x228, 0x3f824000),
                (0x22c, 0xbf0b8000),
                (0x290, 0x3f0f0005),
                (0x294, 0xbf808000),
                (0x298, 0xbce00000),
                (0x29c, 0x3f730000),
                (0x2a0, 0xbf410000),
                (0x2a4, 0x3f0e8000),
                (0x2a8, 0xbf5d8000),
                (0x2ac, 0x3e8a0000),
                (0x400, 0xbf500000),
            ],
        ),
        (
            14,
            &[
                (0x210, 0x3db8001c),
                (0x214, 0x3f190000),
                (0x218, 0xbf050000),
                (0x21c, 0x3e960000),
                (0x220, 0xbf530000),
                (0x224, 0xbee40000),
                (0x228, 0x3f1d0000),
                (0x22c, 0xbf1f0000),
                (0x290, 0x3bfffc80),
                (0x294, 0xbf420000),
                (0x298, 0x3dd00000),
                (0x29c, 0x3eb40000),
                (0x2a0, 0xbe380000),
                (0x2a4, 0x3e640000),
                (0x2a8, 0xbf6b0000),
                (0x2ac, 0xbcc00000),
                (0x400, 0xbee00000),
            ],
        ),
        (
            15,
            &[
                (0x210, 0xbce7ffe8),
                (0x214, 0x3f0c4000),
                (0x218, 0xbebf8000),
                (0x21c, 0x3e4f0000),
                (0x220, 0xbf58c000),
                (0x224, 0xbdcc0000),
                (0x228, 0x3ed88000),
                (0x22c, 0xbf1dc000),
                (0x290, 0x3e79fffa),
                (0x294, 0xbf288000),
                (0x298, 0xbe420000),
                (0x29c, 0x3f088000),
                (0x2a0, 0xbec70000),
                (0x2a4, 0x3dea0000),
                (0x2a8, 0xbf424000),
                (0x2ac, 0xbe360000),
                (0x400, 0x00000000),
            ],
        ),
        (
            16,
            &[
                (0x210, 0xbda30406),
                (0x214, 0x3f376480),
                (0x218, 0xbf0ea380),
                (0x21c, 0x3e748600),
                (0x220, 0xbf847340),
                (0x224, 0xbe948600),
                (0x228, 0x3f4bea80),
                (0x22c, 0xbf376480),
                (0x290, 0x3eb18203),
                (0x294, 0xbf7ec900),
                (0x298, 0x3e351c00),
                (0x29c, 0x3eeb7a00),
                (0x2a0, 0xbece7e00),
                (0x2a4, 0x3f407080),
                (0x2a8, 0xbf65e280),
                (0x2ac, 0xbc876000),
                (0x400, 0x3fb41000),
            ],
        ),
        (
            17,
            &[
                (0x210, 0xbd9b1453),
                (0x214, 0x3f108453),
                (0x218, 0xbefa5da9),
                (0x21c, 0x3e4c9699),
                (0x220, 0xbf48f37c),
                (0x224, 0xbe75351c),
                (0x228, 0x3ecf815c),
                (0x22c, 0xbf2072ed),
                (0x290, 0x3ead8a2a),
                (0x294, 0xbf3108a5),
                (0x298, 0x3d25da94),
                (0x29c, 0x3f09b4b3),
                (0x2a0, 0xbee079c9),
                (0x2a4, 0x3e6d6508),
                (0x2a8, 0xbf59eabc),
                (0x2ac, 0xbe0cb207),
                (0x400, 0x3edd2f57),
                (0x404, 0xbecc292f),
                (0x408, 0x3e6fd246),
            ],
        ),
        (
            18,
            &[
                (0x210, 0xbd4d2321),
                (0x214, 0x3f06f418),
                (0x218, 0xbed5ded9),
                (0x21c, 0x3e235b76),
                (0x220, 0xbf491ca4),
                (0x224, 0xbe217bfa),
                (0x228, 0x3eadf35b),
                (0x22c, 0xbf0a5762),
                (0x290, 0x3e9348c8),
                (0x294, 0xbf1de830),
                (0x298, 0xbdd10934),
                (0x29c, 0x3f1e5245),
                (0x2a0, 0xbef9fd4f),
                (0x2a4, 0x3e8d0462),
                (0x2a8, 0xbf4b9221),
                (0x2ac, 0xbe7dd08e),
                (0x400, 0x3eccb696),
                (0x404, 0xbf5a78de),
                (0x408, 0xbd78e4dc),
                (0x40c, 0x3e5fe4ef),
            ],
        ),
        (
            19,
            &[
                (0x220, 0xbf4a0781),
                (0x224, 0xbe5393ff),
                (0x228, 0x3f0e0483),
                (0x22c, 0xbf0d6580),
                (0x2a0, 0xbef741fe),
                (0x2a4, 0x3e8decfa),
                (0x2a8, 0xbf50c380),
                (0x2ac, 0xbe84b9fc),
                (0x400, 0x3e9b000c),
                (0x404, 0xbf636000),
                (0x408, 0x3f800000),
            ],
        ),
        (
            20,
            &[
                (0x230, 0xbe400019),
                (0x234, 0x3ee00000),
                (0x238, 0xbf100000),
                (0x23c, 0x3d800000),
                (0x2b0, 0x3f300000),
                (0x2b4, 0xbea00000),
                (0x2b8, 0x3ea00000),
                (0x2bc, 0xbf300000),
            ],
        ),
        (
            21,
            &[
                (0x210, 0x3e00001a),
                (0x214, 0xbe800000),
                (0x218, 0xbf200000),
                (0x21c, 0x3f200000),
            ],
        ),
        (
            22,
            &[
                (0x210, 0xbe3c0058),
                (0x214, 0x3f330000),
                (0x218, 0xbeb20000),
                (0x21c, 0x3f090000),
                (0x240, 0xbf0efffb),
                (0x244, 0xbee20000),
                (0x248, 0x3f7f0000),
                (0x24c, 0xbf030000),
                (0x290, 0x3e0c0000),
                (0x294, 0xbfc38000),
                (0x298, 0x3eee0000),
                (0x29c, 0x3df80000),
                (0x2c0, 0x3ca00000),
                (0x2c4, 0x3f570000),
                (0x2c8, 0xbf978000),
                (0x2cc, 0x3eb20000),
            ],
        ),
        (
            23,
            &[
                (0x210, 0xbdbeb594),
                (0x214, 0x3f2093a7),
                (0x218, 0xbec1e969),
                (0x21c, 0x3dd462da),
                (0x220, 0xbf4d0a0d),
                (0x224, 0xbe483396),
                (0x228, 0x3f361f83),
                (0x22c, 0xbf1aa5fb),
                (0x290, 0x3ebf5aca),
                (0x294, 0xbf51274f),
                (0x298, 0xbe385a5b),
                (0x29c, 0x3f3ae74a),
                (0x2a0, 0xbf0d44c0),
                (0x2a4, 0x3e9a9776),
                (0x2a8, 0xbf4a387c),
                (0x2ac, 0x3d45b998),
                (0x400, 0x00000000),
                (0x408, 0xbd1efce4),
                (0x40c, 0x3a02d800),
                (0x410, 0xbedfdf76),
                (0x414, 0x3ebb96e8),
            ],
        ),
        (
            24,
            &[
                (0x210, 0xbe0f75ce),
                (0x214, 0x3f24d5dd),
                (0x218, 0xbefd0d44),
                (0x21c, 0x3dd004d1),
                (0x220, 0xbf3d4857),
                (0x224, 0xbeab54de),
                (0x228, 0x3f31d632),
                (0x22c, 0xbf1d76e8),
                (0x290, 0x3eef75ce),
                (0x294, 0xbf59abb9),
                (0x298, 0x3d50d43c),
                (0x29c, 0x3f3bfecb),
                (0x2a0, 0xbf19f84e),
                (0x2a4, 0x3ed6c2c8),
                (0x2a8, 0xbf61c1bf),
                (0x2ac, 0x3db73263),
                (0x400, 0x3edd2f57),
                (0x404, 0xbecc292f),
                (0x408, 0x3e6fd246),
                (0x40c, 0x3f1544b0),
                (0x410, 0xbf7fe6df),
                (0x414, 0x3e3ec08a),
                (0x418, 0x3ea56e70),
            ],
        ),
        (
            25,
            &[
                (0x210, 0x3d07868b),
                (0x214, 0x3f21f98f),
                (0x218, 0xbedd3159),
                (0x21c, 0x3e2a49d9),
                (0x220, 0xbf833fe2),
                (0x224, 0xbd6a944c),
                (0x228, 0x3f0e8594),
                (0x22c, 0xbf36e5b5),
                (0x290, 0x3df87975),
                (0x294, 0xbf53f31e),
                (0x298, 0xbd967538),
                (0x29c, 0x3f1adb14),
                (0x2a0, 0xbf0a7524),
                (0x2a4, 0x3efaa78a),
                (0x2a8, 0xbf46cba4),
                (0x2ac, 0xbe2d040d),
                (0x400, 0x00000000),
                (0x40c, 0x3f10d3c1),
                (0x410, 0x00000000),
                (0x418, 0x3d0cd450),
            ],
        ),
        (
            26,
            &[
                (0x210, 0xbdf1c2d1),
                (0x214, 0x3f35abb1),
                (0x218, 0xbef2693b),
                (0x21c, 0x3df50908),
                (0x220, 0xbf547ad5),
                (0x224, 0xbe916c8b),
                (0x228, 0x3f65bcd9),
                (0x22c, 0xbf17a15a),
                (0x290, 0x3ed8e168),
                (0x294, 0xbf7b5760),
                (0x298, 0x3c1a4eb3),
                (0x29c, 0x3f32bdbe),
                (0x2a0, 0xbf0f9739),
                (0x2a4, 0x3f1c4948),
                (0x2a8, 0xbf5ac7e1),
                (0x2ac, 0xbe67090e),
                (0x400, 0x3e358000),
                (0x404, 0xbf2f49f6),
                (0x408, 0x3ea00000),
                (0x40c, 0x3dfc0cfc),
            ],
        ),
        (
            27,
            &[
                (0x310, 0x3f36b280),
                (0x314, 0xbf0c4680),
                (0x318, 0x3efdc301),
                (0x31c, 0xbf441780),
                (0x320, 0x3e802863),
                (0x324, 0x3f19069a),
                (0x328, 0xbf197fda),
                (0x32c, 0x3ed5c1ae),
                (0x390, 0xbf123b80),
                (0x394, 0x3e105e00),
                (0x398, 0x3f205f80),
                (0x39c, 0xbe929b00),
                (0x3a0, 0x3ea3eaf8),
                (0x3a4, 0xbf67a217),
                (0x3a8, 0x3dad6be4),
                (0x3ac, 0x3efd9144),
                (0x400, 0xbe7b8004),
                (0x404, 0xbf1231e2),
            ],
        ),
        (
            28,
            &[
                (0x210, 0x3d856000),
                (0x214, 0x3eed2d56),
                (0x218, 0xbeca3800),
                (0x21c, 0x3b4eaac0),
                (0x220, 0xbf5ae400),
                (0x224, 0x3c34aaa0),
                (0x228, 0x3d5fc000),
                (0x22c, 0xbf0c5eab),
                (0x290, 0x3d6a8000),
                (0x294, 0xbefa5aab),
                (0x298, 0xbe172000),
                (0x29c, 0x3f6e62aa),
                (0x2a0, 0xbe008aac),
                (0x2a4, 0xbcce8000),
                (0x2a8, 0xbf3dd955),
                (0x2ac, 0xbeffbaaa),
                (0x400, 0x3f9c2aaa),
            ],
        ),
        (
            29,
            &[
                (0x210, 0xbf84be02),
                (0x214, 0x40330528),
                (0x218, 0xbff32d63),
                (0x21c, 0xbe8f844c),
                (0x220, 0xbf9898c5),
                (0x224, 0xc091c83c),
                (0x228, 0x4022f9c6),
                (0x22c, 0xbfd4fbd9),
                (0x290, 0x4010be02),
                (0x294, 0xc0a50528),
                (0x298, 0x40372d63),
                (0x29c, 0x3fbfc226),
                (0x2a0, 0x40df883c),
                (0x2a4, 0x40018528),
                (0x2a8, 0xc0004f76),
                (0x2ac, 0x404ced63),
                (0x400, 0x40600000),
                (0x404, 0xc1a70000),
                (0x408, 0x40ac73b0),
            ],
        ),
        (
            30,
            &[
                (0x210, 0x3d9e7930),
                (0x214, 0x3ec7426c),
                (0x218, 0xbf05c5f2),
                (0x21c, 0x3ec1d49c),
                (0x220, 0xbf6f43a2),
                (0x224, 0x3d283240),
                (0x228, 0xbe231468),
                (0x22c, 0xbf2c5386),
                (0x290, 0x3d061b40),
                (0x294, 0xbeae84d8),
                (0x298, 0x3ddc5f20),
                (0x29c, 0x3e38ad90),
                (0x2a0, 0xbdda3460),
                (0x2a4, 0xbe82e744),
                (0x2a8, 0xbf5f385e),
                (0x2ac, 0x3e7bbf10),
                (0x400, 0xbd559800),
                (0x404, 0x3ecc0900),
                (0x408, 0x3d107800),
            ],
        ),
        (
            31,
            &[
                (0x210, 0xbe11cee2),
                (0x214, 0x3f0c404d),
                (0x218, 0xbeb25e06),
                (0x21c, 0x3ec3ae5d),
                (0x220, 0xbf980455),
                (0x224, 0xbe8cd824),
                (0x228, 0xbe5303e6),
                (0x22c, 0xbef049c5),
                (0x290, 0x3ef1cee2),
                (0x294, 0xbf28809b),
                (0x298, 0xbe7687e6),
                (0x29c, 0x3e31468c),
                (0x2a0, 0xbe70b528),
                (0x2a4, 0xbe8d030f),
                (0x2a8, 0xbf28ba67),
                (0x2ac, 0x3e3c8e98),
                (0x400, 0x3fc92993),
                (0x404, 0xbe613e80),
                (0x408, 0xbf80e68b),
                (0x40c, 0x3fa296a1),
                (0x410, 0xbf8604d4),
                (0x414, 0x3f5e9c08),
                (0x418, 0xbf82b746),
            ],
        ),
        (
            32,
            &[
                (0x210, 0xbe2d7e9b),
                (0x214, 0x3f2cec63),
                (0x218, 0xbf0edf3c),
                (0x21c, 0x3db7ab48),
                (0x220, 0xbf30be8e),
                (0x224, 0xbed228b5),
                (0x228, 0x3f434595),
                (0x22c, 0xbedcd2fe),
                (0x290, 0x3f06bf4d),
                (0x294, 0xbf69d8c8),
                (0x298, 0x3e36f9dd),
                (0x29c, 0x3f42152e),
                (0x2a0, 0xbf246a3d),
                (0x2a4, 0x3f0a44c1),
                (0x2a8, 0xbf6db4b4),
                (0x2ac, 0xbebf803f),
                (0x400, 0x3f718a80),
                (0x404, 0xbf85cc24),
                (0x408, 0xbe428536),
            ],
        ),
        (
            33,
            &[
                (0x210, 0x3dc84f60),
                (0x214, 0x3ef94b01),
                (0x218, 0xbede68a5),
                (0x21c, 0x3dccc978),
                (0x220, 0xbf566ce9),
                (0x224, 0x3d8d6010),
                (0x228, 0x3e61d230),
                (0x22c, 0xbf0e5da4),
                (0x290, 0x3cc89674),
                (0x294, 0xbf1307a3),
                (0x298, 0xbdda9ff0),
                (0x29c, 0x3f45262d),
                (0x2a0, 0xbf2d3574),
                (0x2a4, 0x3d8512e2),
                (0x2a8, 0xbf3331a2),
                (0x2ac, 0xbeba2dd0),
                (0x400, 0x3f800000),
                (0x404, 0xc0bbb259),
                (0x40c, 0x3e9565c5),
                (0x410, 0x3f800000),
                (0x418, 0xbf39be8c),
            ],
        ),
        (
            34,
            &[
                (0x210, 0xbd02eb68),
                (0x214, 0x3f018999),
                (0x218, 0xbee11332),
                (0x21c, 0x3e66badc),
                (0x220, 0xbf58e47b),
                (0x224, 0xbe17bd68),
                (0x228, 0x3e7107a4),
                (0x22c, 0xbf13528f),
                (0x290, 0x3e80bada),
                (0x294, 0xbf131332),
                (0x298, 0xbd6ecce1),
                (0x29c, 0x3ef94524),
                (0x2a0, 0xbec468fc),
                (0x2a4, 0x3d4f7acf),
                (0x2a8, 0xbf4dc0a3),
                (0x2ac, 0xbdb88531),
                (0x400, 0x3f200000),
                (0x404, 0xbf60bf08),
                (0x408, 0xbcdc27f4),
                (0x40c, 0x3f800000),
            ],
        ),
        (
            35,
            &[
                (0x210, 0xbdaf4988),
                (0x214, 0x3f180e9c),
                (0x218, 0xbf154fc3),
                (0x21c, 0x3e7c1776),
                (0x220, 0xbf330257),
                (0x224, 0xbea182db),
                (0x228, 0x3ed18a82),
                (0x22c, 0xbf3fc74a),
                (0x290, 0x3eb7a4c4),
                (0x294, 0xbf401d38),
                (0x298, 0x3e6a7e1e),
                (0x29c, 0x3ee3e88a),
                (0x2a0, 0xbecdc6a4),
                (0x2a4, 0x3ecf24d2),
                (0x2a8, 0xbf6ad83d),
                (0x2ac, 0xbf0c0538),
                (0x400, 0x3fa04002),
                (0x404, 0x00000000),
                (0x40c, 0x3e79c344),
            ],
        ),
        (
            36,
            &[
                (0x210, 0xbf0da24c),
                (0x214, 0xbef5f508),
                (0x218, 0x3e1b02c0),
                (0x21c, 0xbe2770bd),
                (0x220, 0xbf4a0781),
                (0x224, 0xbe5393ff),
                (0x228, 0x3f0e0483),
                (0x22c, 0xbf0d6580),
                (0x230, 0x3e400019),
                (0x234, 0xbf500000),
                (0x238, 0xbe400000),
                (0x23c, 0x3ee00000),
                (0x240, 0xbe1bfffc),
                (0x244, 0x3e8a0000),
                (0x248, 0x3da80000),
                (0x24c, 0xbdd80000),
                (0x290, 0x3f83868f),
                (0x294, 0xbf87d417),
                (0x298, 0x3e806181),
                (0x29c, 0x3f82d8cf),
                (0x2a0, 0xbef741fe),
                (0x2a4, 0x3e8decfa),
                (0x2a8, 0xbf50c380),
                (0x2ac, 0xbe84b9fc),
                (0x2b0, 0xbf100000),
                (0x2b4, 0x3d800000),
                (0x2b8, 0x3f300000),
                (0x2bc, 0xbea00000),
                (0x2c0, 0xbf590000),
                (0x2c4, 0x3f2f0000),
                (0x2c8, 0xbeda0000),
                (0x2cc, 0xbf050000),
                (0x400, 0x3e9b000c),
                (0x404, 0xbf636000),
                (0x408, 0x3f800000),
            ],
        ),
    ];
    const GOLDEN_SELECTOR_ZERO: [(usize, &[Change]); 3] = [
        (
            5,
            &[
                (0x210, 0x3d64001c),
                (0x214, 0x3f024000),
                (0x218, 0xbec48000),
                (0x21c, 0x3d8e0000),
                (0x220, 0xbf52c000),
                (0x224, 0x3bc00000),
                (0x228, 0x3e590000),
                (0x22c, 0xbf07c000),
                (0x290, 0x3d9bffe4),
                (0x294, 0xbf148000),
                (0x298, 0xbe2e0000),
                (0x29c, 0x3f4c8000),
                (0x2a0, 0xbf378000),
                (0x2a4, 0x3dba0000),
                (0x2a8, 0xbf3cc000),
                (0x2ac, 0xbec30000),
                (0x400, 0x3f800000),
                (0x404, 0xc0a25628),
                (0x408, 0xbec00000),
            ],
        ),
        (
            10,
            &[
                (0x220, 0xbf373ffe),
                (0x224, 0xbe2e0000),
                (0x228, 0x3e4d0000),
                (0x22c, 0xbf004000),
                (0x2a0, 0xbefb0000),
                (0x2a4, 0x3f07c000),
                (0x2a8, 0xbf494000),
                (0x2ac, 0xbe9f0000),
                (0x400, 0xbe800000),
                (0x404, 0xc0303b2c),
                (0x408, 0x3f200000),
            ],
        ),
        (
            33,
            &[
                (0x210, 0x3dc034b1),
                (0x214, 0x3efab994),
                (0x218, 0xbedbfef9),
                (0x21c, 0x3dc64089),
                (0x220, 0xbf562e23),
                (0x224, 0x3d84be6e),
                (0x228, 0x3e613b88),
                (0x22c, 0xbf0deca5),
                (0x290, 0x3ce4d600),
                (0x294, 0xbf1320bf),
                (0x298, 0xbde34192),
                (0x29c, 0x3f45a3b9),
                (0x2a0, 0xbf2de538),
                (0x2a4, 0x3d889ad3),
                (0x2a8, 0xbf33d4d8),
                (0x2ac, 0xbebac478),
                (0x400, 0x3f800000),
                (0x404, 0xc0a25628),
                (0x408, 0xbec00000),
                (0x40c, 0x3ea1c5c4),
                (0x410, 0x3f800000),
                (0x414, 0x3e800000),
                (0x418, 0xbf3ccb0a),
            ],
        ),
    ];
    const GOLDEN_WORD: [(usize, &[Change]); 2] = [
        (
            21,
            &[
                (0x210, 0x3ed00021),
                (0x214, 0xbfbc0000),
                (0x218, 0xbf680000),
                (0x21c, 0x3fa40000),
                (0x240, 0xbecfffec),
                (0x244, 0xbfa40000),
                (0x248, 0x3e900000),
                (0x24c, 0x3e600000),
            ],
        ),
        (
            36,
            &[
                (0x210, 0xbfcad13a),
                (0x214, 0xbf12fa84),
                (0x218, 0x3f7ec0b0),
                (0x21c, 0xbf51dc2f),
                (0x220, 0xbf4a0781),
                (0x224, 0xbe5393ff),
                (0x228, 0x3f0e0483),
                (0x22c, 0xbf0d6580),
                (0x230, 0x3e400019),
                (0x234, 0xbf500000),
                (0x238, 0xbe400000),
                (0x23c, 0x3ee00000),
                (0x240, 0xbf978014),
                (0x244, 0x3e340000),
                (0x248, 0x3f6d0000),
                (0x24c, 0xbf430000),
                (0x290, 0x3f83868f),
                (0x294, 0xbf87d417),
                (0x298, 0x3e806181),
                (0x29c, 0x3f82d8cf),
                (0x2a0, 0xbef741fe),
                (0x2a4, 0x3e8decfa),
                (0x2a8, 0xbf50c380),
                (0x2ac, 0xbe84b9fc),
                (0x2b0, 0xbf100000),
                (0x2b4, 0x3d800000),
                (0x2b8, 0x3f300000),
                (0x2bc, 0xbea00000),
                (0x2c0, 0xbf590000),
                (0x2c4, 0x3f2f0000),
                (0x2c8, 0xbeda0000),
                (0x2cc, 0xbf050000),
                (0x400, 0x3e9b000c),
                (0x404, 0xbf636000),
                (0x408, 0x3f800000),
            ],
        ),
    ];
    const GOLDEN_WORD_SELECTOR_ZERO: [(usize, &[Change]); 2] = [
        (
            21,
            &[
                (0x210, 0x3ed00021),
                (0x214, 0xbfbc0000),
                (0x218, 0xbf680000),
                (0x21c, 0x3fa40000),
                (0x240, 0xbecfffec),
                (0x244, 0xbfa40000),
                (0x248, 0x3e900000),
                (0x24c, 0x3e600000),
            ],
        ),
        (
            36,
            &[
                (0x210, 0xbfcad13a),
                (0x214, 0xbf12fa84),
                (0x218, 0x3f7ec0b0),
                (0x21c, 0xbf51dc2f),
                (0x220, 0xbf4a0781),
                (0x224, 0xbe5393ff),
                (0x228, 0x3f0e0483),
                (0x22c, 0xbf0d6580),
                (0x230, 0x3e400019),
                (0x234, 0xbf500000),
                (0x238, 0xbe400000),
                (0x23c, 0x3ee00000),
                (0x240, 0xbf978014),
                (0x244, 0x3e340000),
                (0x248, 0x3f6d0000),
                (0x24c, 0xbf430000),
                (0x290, 0x3f83868f),
                (0x294, 0xbf87d417),
                (0x298, 0x3e806181),
                (0x29c, 0x3f82d8cf),
                (0x2a0, 0xbef741fe),
                (0x2a4, 0x3e8decfa),
                (0x2a8, 0xbf50c380),
                (0x2ac, 0xbe84b9fc),
                (0x2b0, 0xbf100000),
                (0x2b4, 0x3d800000),
                (0x2b8, 0x3f300000),
                (0x2bc, 0xbea00000),
                (0x2c0, 0xbf590000),
                (0x2c4, 0x3f2f0000),
                (0x2c8, 0xbeda0000),
                (0x2cc, 0xbf050000),
                (0x400, 0x3e9b000c),
                (0x404, 0xbf636000),
                (0x408, 0x3f800000),
            ],
        ),
    ];

    type Golden = [(usize, &'static [Change])];

    /// The engine with the byte table, the license flag and doubles for the vector helpers the
    /// record types call, each as the exe's function behaves (`005dbf20`: the float into the
    /// first lane, the others cleared; `00c7f750`: the matrix times the vector; `00c6ccc0`,
    /// `00c6cce0` and `00891060`: the first lane of `a` minus, plus or times that of `b`).
    fn solver_engine() -> Engine {
        let mut e = engine();
        for (index, byte) in ADVANCE_BY_TYPE.iter().enumerate() {
            e.mem.set_u8(BYTE_TABLE + index as u32, *byte);
        }
        e.mem.set_u8(LICENSE_VALID_FLAG, 1);
        e.register_double(VECTOR_FROM_FLOAT, |e, a| {
            store_vec(e, a[0], [f32::from_bits(a[1]), 0.0, 0.0, 0.0]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register_double(SET_MUL3, |e, a| {
            let columns = [0, 0x10, 0x20].map(|offset| load_vec(e, a[1] + offset));
            let v = load_vec(e, a[2]);
            let by_y = mul(splat(v[1]), columns[1]);
            let by_x = mul(splat(v[0]), columns[0]);
            let by_z = mul(splat(v[2]), columns[2]);
            store_vec(e, a[0], add(add(by_y, by_x), by_z));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let first_lane_operations: [(u32, fn(f32, f32) -> f32); 3] = [
            (VECTOR_SUBTRACT_FIRST_LANE, |x, y| x - y),
            (VECTOR_ADD_FIRST_LANE, |x, y| x + y),
            (VECTOR_MULTIPLY_FIRST_LANE, |x, y| x * y),
        ];
        for (address, operation) in first_lane_operations {
            e.register_double(address, move |e, a| {
                let mut v = load_vec(e, a[1]);
                v[0] = operation(v[0], e.mem.f32(a[2]));
                store_vec(e, a[0], v);
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        e
    }

    /// The recorded run of a scenario in the first table that has it.
    fn expected(tables: &[&Golden], index: usize) -> &'static [Change] {
        tables
            .iter()
            .find_map(|table| table.iter().find(|entry| entry.0 == index))
            .map(|entry| entry.1)
            .expect("a table records the scenario")
    }

    /// Runs `steps` through the first function and returns what changed.
    fn first_changes(steps: &[Step]) -> Vec<Change> {
        let mut e = solver_engine();
        let fx = fixture(&mut e, steps);
        let before = snapshot(&e, &fx);
        e.call(0x00d8_aac0, &args![fx.info, fx.schema, fx.bodies, fx.accum]);
        changes(&before, &snapshot(&e, &fx))
    }

    /// Runs `steps` through the second function and returns what changed and its answer.
    fn second_changes(steps: &[Step], word: u32, selector: u32) -> (Vec<Change>, u32) {
        let mut e = solver_engine();
        let fx = fixture(&mut e, steps);
        let before = snapshot(&e, &fx);
        let answer = e
            .call(
                0x00d8_fb70,
                &args![fx.info, word, selector, fx.schema, fx.bodies, fx.accum],
            )
            .u32();
        (changes(&before, &snapshot(&e, &fx)), answer)
    }

    #[test]
    fn step_jacobians_follow_the_recorded_runs() {
        for (index, steps) in SCENARIOS.iter().enumerate() {
            assert_eq!(
                first_changes(steps),
                expected(&[&GOLDEN_FIRST], index),
                "scenario {index}"
            );
        }
    }

    #[test]
    fn by_the_steps_with_the_float_zero_is_the_first_function() {
        for (index, steps) in SCENARIOS.iter().enumerate() {
            let (changed, answer) = second_changes(steps, 0, 1);
            assert_eq!(answer, 1);
            assert_eq!(
                changed,
                expected(&[&GOLDEN_FIRST], index),
                "scenario {index}"
            );
        }
    }

    #[test]
    fn by_the_steps_selector_zero_changes_the_rows_that_read_the_float() {
        for (index, steps) in SCENARIOS.iter().enumerate() {
            let (changed, answer) = second_changes(steps, 0, 0);
            assert_eq!(answer, 1);
            assert_eq!(
                changed,
                expected(&[&GOLDEN_SELECTOR_ZERO, &GOLDEN_FIRST], index),
                "scenario {index}"
            );
        }
        // Rows 10 and 15 (scenarios 5 and 10), and the sequence with row 10 (33), differ.
        let differing: Vec<usize> = GOLDEN_SELECTOR_ZERO.iter().map(|entry| entry.0).collect();
        assert_eq!(differing, [5, 10, 33]);
    }

    #[test]
    fn by_the_steps_word_reaches_the_impulse_row() {
        for (index, steps) in SCENARIOS.iter().enumerate() {
            let (changed, _) = second_changes(steps, 3, 1);
            assert_eq!(
                changed,
                expected(&[&GOLDEN_WORD, &GOLDEN_FIRST], index),
                "scenario {index}"
            );
            let (changed, _) = second_changes(steps, 3, 0);
            assert_eq!(
                changed,
                expected(
                    &[
                        &GOLDEN_WORD_SELECTOR_ZERO,
                        &GOLDEN_SELECTOR_ZERO,
                        &GOLDEN_FIRST
                    ],
                    index
                ),
                "scenario {index}"
            );
        }
        // Only row 26 (scenario 21) and the sequence that ends with it (36) use the word.
        let differing: Vec<usize> = GOLDEN_WORD.iter().map(|entry| entry.0).collect();
        assert_eq!(differing, [21, 36]);
    }

    #[test]
    fn body_switch_selects_the_named_bodies() {
        // Scenario 27 selects bodies 2 and 3, skips two records and runs rows 5 and 12.
        let changed = first_changes(SCENARIOS[27]);
        assert!(!changed.is_empty());
        for (offset, _) in changed {
            assert!(
                offset >= ACCUM_AT || (BODIES_AT + 0x100..ACCUM_AT).contains(&offset),
                "{offset:#x}"
            );
        }
    }

    /// How many times the log recorded a call to `addr`, and with which words.
    fn logged_calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn by_the_steps_runs_the_license_check_once() {
        let mut e = solver_engine();
        e.mem.set_u8(LICENSE_VALID_FLAG, 0);
        e.register_double(LICENSE_CHECK, |e, a| {
            e.mem.set_u8(a[0], 1);
            Ret::default()
        });
        e.call_log = Some(Vec::new());
        let fx = fixture(&mut e, &[Step::Row(5)]);
        let before = snapshot(&e, &fx);
        let answer = e
            .call(
                0x00d8_fb70,
                &args![fx.info, 0u32, 1u32, fx.schema, fx.bodies, fx.accum],
            )
            .u32();
        assert_eq!(answer, 1);
        assert_eq!(
            changes(&before, &snapshot(&e, &fx)),
            expected(&[&GOLDEN_FIRST], 0)
        );
        assert_eq!(logged_calls(&e, LICENSE_CHECK), [vec![LICENSE_VALID_FLAG]]);
        // With the flag set the check is not run again.
        e.call(
            0x00d8_fb70,
            &args![fx.info, 0u32, 1u32, fx.schema, fx.bodies, fx.accum],
        );
        assert_eq!(logged_calls(&e, LICENSE_CHECK).len(), 1);
    }

    #[test]
    fn by_the_steps_answers_zero_without_the_license() {
        let mut e = solver_engine();
        e.mem.set_u8(LICENSE_VALID_FLAG, 0);
        e.register_double(LICENSE_CHECK, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        let fx = fixture(&mut e, &[Step::Row(5)]);
        let before = snapshot(&e, &fx);
        let answer = e
            .call(
                0x00d8_fb70,
                &args![fx.info, 0u32, 1u32, fx.schema, fx.bodies, fx.accum],
            )
            .u32();
        assert_eq!(answer, 0);
        assert_eq!(snapshot(&e, &fx), before);
        assert_eq!(logged_calls(&e, LICENSE_CHECK).len(), 1);
    }

    /// A schema whose first record after the body selection has this type byte.
    fn fixture_with_type_byte(e: &mut Engine, kind: u8) -> Fixture {
        let fx = fixture(e, &[Step::SkipSixteen]);
        e.mem.set_u8(fx.schema + 0x10, kind);
        fx
    }

    #[test]
    #[should_panic(expected = "INT3 at 00d8cf9d")]
    fn step_jacobians_stop_at_the_unused_type_four() {
        let mut e = solver_engine();
        let fx = fixture_with_type_byte(&mut e, 4);
        e.call(0x00d8_aac0, &args![fx.info, fx.schema, fx.bodies, fx.accum]);
    }

    #[test]
    #[should_panic(expected = "INT3 at 00d8cf9d")]
    fn step_jacobians_stop_at_a_negative_type() {
        let mut e = solver_engine();
        let fx = fixture_with_type_byte(&mut e, 0xf0);
        e.call(0x00d8_aac0, &args![fx.info, fx.schema, fx.bodies, fx.accum]);
    }

    #[test]
    #[should_panic(expected = "INT3 at 00d92129")]
    fn by_the_steps_stop_at_type_twenty_eight() {
        let mut e = solver_engine();
        let fx = fixture_with_type_byte(&mut e, 28);
        e.call(
            0x00d8_fb70,
            &args![fx.info, 0u32, 1u32, fx.schema, fx.bodies, fx.accum],
        );
    }

    #[test]
    #[should_panic(expected = "INT3 at 00d92129")]
    fn by_the_steps_stop_above_type_thirty_one() {
        let mut e = solver_engine();
        let fx = fixture_with_type_byte(&mut e, 32);
        e.call(
            0x00d8_fb70,
            &args![fx.info, 0u32, 1u32, fx.schema, fx.bodies, fx.accum],
        );
    }

    /// The powered chain solver's helpers, none of them translated: doubles that change nothing.
    const POWERED_CHAIN_HELPERS: [u32; 7] = [
        FN_00DF0D30,
        FN_00DF0810,
        FN_00DF11A0,
        FN_00DF1430,
        FN_00DF2040,
        FN_00DF1060,
        FN_00DF1260,
    ];

    fn powered_chain_engine() -> Engine {
        let mut e = solver_engine();
        for helper in POWERED_CHAIN_HELPERS {
            e.register_double(helper, |_, _| Ret::default());
        }
        e.call_log = Some(Vec::new());
        e
    }

    #[test]
    fn powered_chain_record_goes_to_the_powered_solver() {
        let mut e = powered_chain_engine();
        let fx = fixture(&mut e, &[Step::Chain(31, 1), Step::Row(5)]);
        let before = snapshot(&e, &fx);
        e.call(0x00d8_aac0, &args![fx.info, fx.schema, fx.bodies, fx.accum]);
        // The solver's context block (first word: the solver info, `+0x10`: the links, 0x20
        // bytes into the record, `+0x1c`: the bodies) is the first argument of its first helper.
        let calls = logged_calls(&e, FN_00DF0D30);
        assert_eq!(calls.len(), 1);
        let context = calls[0][0];
        assert_eq!(e.mem.u32(context), fx.info);
        assert_eq!(e.mem.u32(context + 0x10), fx.schema + 0x10 + 0x20);
        assert_eq!(e.mem.u32(context + 0x1c), fx.bodies);
        // The accumulators moved on by six floats for the one link: the row after the record
        // writes the seventh one.
        let changed = changes(&before, &snapshot(&e, &fx));
        assert!(changed.iter().any(|(offset, _)| *offset == ACCUM_AT + 24));
    }

    #[test]
    fn powered_chain_step_block_carries_the_selector_float() {
        // The solver adds `factor * value` to words of the record; the factor is the float at
        // `+4` of the block the function passes: `0.0` for a non-zero selector, `1.0` for zero.
        let mut results = Vec::new();
        for selector in [1u32, 0] {
            let mut e = powered_chain_engine();
            let fx = fixture(&mut e, &[Step::Chain(31, 1)]);
            let slot = fx.schema + 0x10 + 0x20 + 0x90 + 0x1c;
            let before = e.mem.f32(slot);
            e.call(
                0x00d8_fb70,
                &args![fx.info, 0u32, selector, fx.schema, fx.bodies, fx.accum],
            );
            results.push((before, e.mem.f32(slot)));
        }
        assert_eq!(results[0].0, results[0].1);
        assert_ne!(results[1].0, results[1].1);
    }
}
