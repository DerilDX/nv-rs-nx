//! `Havok SDK/hkpsolver.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vector code is SSE: every lane is a single-precision operation, bit for bit as the exe
//! does it. The scalar parts that go through the x87 stack (`FLD` / `FMUL` / `FSTP`) load
//! through `x87_quiet`, compute in `double` and round to `float` where the exe stores.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, load_bits, load_vec, store_vec, sum3, zip_with, Vec4,
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
}
