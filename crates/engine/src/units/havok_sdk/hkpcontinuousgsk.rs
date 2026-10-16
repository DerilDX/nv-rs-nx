//! `Havok SDK/hkpcontinuousgsk.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use crate::prelude::*;

/// Four SSE lanes as bits.
type Lanes = [u32; 4];

/// The vector of ones (four floats).
const VECTOR_ONES: u32 = 0x010e_c7f0;
/// The sign mask (`0x80000000` in four lanes).
const SIGN_MASK: u32 = 0x010c_44d0;
/// The double the velocity error is scaled by (`1.2f` widened).
const VELOCITY_DIVISOR: u32 = 0x0102_90f0;
/// The double a quarter.
const QUARTER: u32 = 0x0102_90b0;
/// The doubles one half and two.
const HALF: u32 = 0x0101_1588;
const TWO: u32 = 0x0101_1590;
/// `_CIsqrt` (`00ec6040`): the square root of its argument (a double).
const X87_SQRT: u32 = 0x00ec_6040;
/// The doubles the interpolation fraction is clamped against (`0.1f` and `0.9f` widened).
const LOWER_FRACTION: u32 = 0x0101_ffa0;
const UPPER_FRACTION: u32 = 0x0106_b9e8;
/// The floats the fraction is set to when it is outside (`0.1f` and `0.9f`).
const LOWER_FRACTION_FLOAT: u32 = 0x0101_e2bc;
const UPPER_FRACTION_FLOAT: u32 = 0x0101_77e8;

/// `hkSweptTransformUtil::lerp2Ha` (`00cf1710`, cdecl: the swept transform at `+0x40` of a
/// body, the start time, the time step, the output transform).
const INTERPOLATE_TRANSFORM: u32 = 0x00cf_1710;
/// `hkTransform::setMulInverseMul` (`00cb22f0`, thiscall: the result, two transforms).
const SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
/// `hkVector4::setRotatedDir` (`00c7f700`, thiscall: the result, a transform, a vector).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// `hkVector4::setTransformedPos` (`00c7f640`, thiscall: the result, a transform, a vector).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// `004b4ab0` (thiscall: the result, a transform, a vector); no name known.
const TRANSFORM_UNNAMED_004B4AB0: u32 = 0x004b_4ab0;
/// `hkpGsk::getClosestFeature` (`00daad40`, thiscall on the `hkpGsk`: two shapes, a
/// transform, the output direction vector whose fourth lane is the distance).
const GET_CLOSEST_FEATURE: u32 = 0x00da_ad40;
/// The memory router (`00c85750`, no arguments): the object whose first word is the cursor
/// of a stack-like allocation area.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The double added to a sum before it is inverted (`1.1920929e-07`).
const EPSILON: u32 = 0x010c_9300;

/// `x87` arithmetic on two stack values with the hardware NaN rules: with a NaN operand the one
/// with the larger significand wins (a NaN against a number: the NaN), made quiet.
fn x87_op(first: f64, second: f64, op: impl Fn(f64, f64) -> f64) -> f64 {
    let quiet = 1u64 << 51;
    let significand = 0x000f_ffff_ffff_ffffu64;
    match (first.is_nan(), second.is_nan()) {
        (true, true) => {
            if first.to_bits() & significand >= second.to_bits() & significand {
                f64::from_bits(first.to_bits() | quiet)
            } else {
                f64::from_bits(second.to_bits() | quiet)
            }
        }
        (true, false) => f64::from_bits(first.to_bits() | quiet),
        (false, true) => f64::from_bits(second.to_bits() | quiet),
        (false, false) => op(first, second),
    }
}

fn x87_add(first: f64, second: f64) -> f64 {
    x87_op(first, second, |a, b| a + b)
}

fn x87_sub(first: f64, second: f64) -> f64 {
    x87_op(first, second, |a, b| a - b)
}

fn x87_mul(first: f64, second: f64) -> f64 {
    x87_op(first, second, |a, b| a * b)
}

fn x87_div(first: f64, second: f64) -> f64 {
    x87_op(first, second, |a, b| a / b)
}

/// `FLD dword`: the float's bits (a signalling NaN quieted) as the stack value.
fn x87_load32(bits: u32) -> f64 {
    f64::from(f32::from_bits(x87_quiet_bits(bits)))
}

/// `FSTP dword`: the stack value rounded to a float, as its bits.
fn x87_store32(value: f64) -> u32 {
    (value as f32).to_bits()
}

/// One lane of an SSE arithmetic instruction: a NaN in the destination operand wins
/// (quieted), then a NaN in the source operand, otherwise `op`.
fn sse(dest: u32, source: u32, op: fn(f32, f32) -> f32) -> u32 {
    let quiet = 0x0040_0000;
    if f32::from_bits(dest).is_nan() {
        dest | quiet
    } else if f32::from_bits(source).is_nan() {
        source | quiet
    } else {
        op(f32::from_bits(dest), f32::from_bits(source)).to_bits()
    }
}

fn lanes_op(dest: Lanes, source: Lanes, op: fn(f32, f32) -> f32) -> Lanes {
    [0, 1, 2, 3].map(|lane| sse(dest[lane], source[lane], op))
}

fn add(a: f32, b: f32) -> f32 {
    a + b
}

fn sub(a: f32, b: f32) -> f32 {
    a - b
}

fn mul(a: f32, b: f32) -> f32 {
    a * b
}

/// The lane sum `x2 + (x1 + x0)` the code builds with two `ADDSS` over shuffled copies.
fn lane_sum(lanes: Lanes) -> u32 {
    let low = sse(lanes[1], lanes[0], add);
    sse(lanes[2], low, add)
}

/// `SHUFPS` of a register with itself: lane `n` of the result is lane `order[n]`.
fn shuffle(lanes: Lanes, order: [usize; 4]) -> Lanes {
    order.map(|lane| lanes[lane])
}

/// A `MOVAPS` load of four lanes.
fn load_lanes(e: &Engine, addr: u32) -> Lanes {
    [0, 1, 2, 3].map(|lane| e.mem.u32(addr + 4 * lane))
}

/// A `MOVAPS` store of four lanes.
fn store_lanes(e: &mut Engine, addr: u32, lanes: Lanes) {
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_u32(addr + 4 * lane as u32, value);
    }
}

/// Copies the four words at `from` to `to`.
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    let lanes = load_lanes(e, from);
    store_lanes(e, to, lanes);
}

/// The columns of three transform rows, as the unpack and move shuffles build them (the
/// fourth lanes are `row2[1]`, `row2[3]` and `row2[3]`).
fn transpose_rows(rows: [Lanes; 3]) -> [Lanes; 3] {
    let [row0, row1, row2] = rows;
    [
        [row0[0], row1[0], row2[0], row2[1]],
        [row0[1], row1[1], row2[1], row2[3]],
        [row0[2], row1[2], row2[2], row2[3]],
    ]
}

/// `(time - motion[0x4c]) * motion[0x5c]` on the x87 stack, rounded to a float: how far
/// through its step a motion record is at `time`.
fn motion_fraction(e: &Engine, motion: u32, time: f64) -> u32 {
    let start = x87_load32(e.mem.u32(motion + 0x4c));
    let scale = x87_load32(e.mem.u32(motion + 0x5c));
    x87_store32(x87_mul(x87_sub(time, start), scale))
}

/// The motion record's interpolated position: `(1 - f) * record[+0x40] + record[+0x50] * f`
/// in every lane.
fn blended_position(e: &Engine, motion: u32, fraction: u32) -> Lanes {
    let spread = [fraction; 4];
    let ones = load_lanes(e, VECTOR_ONES);
    let from = lanes_op(
        lanes_op(ones, spread, sub),
        load_lanes(e, motion + 0x40),
        mul,
    );
    let to = lanes_op(load_lanes(e, motion + 0x50), spread, mul);
    lanes_op(from, to, add)
}

/// `axis x offset` with the shuffles `0xc9` and `0xd2`.
fn cross(axis: Lanes, offset: Lanes) -> Lanes {
    let forward = lanes_op(
        shuffle(axis, [1, 2, 0, 3]),
        shuffle(offset, [2, 0, 1, 3]),
        mul,
    );
    let backward = lanes_op(
        shuffle(axis, [2, 0, 1, 3]),
        shuffle(offset, [1, 2, 0, 3]),
        mul,
    );
    lanes_op(forward, backward, sub)
}

/// `(axis x offset) . direction`, as the lane sum.
fn cross_dot(axis: Lanes, offset: Lanes, direction: Lanes) -> u32 {
    lane_sum(lanes_op(cross(axis, offset), direction, mul))
}

/// A dot product scaled by the velocity divisor on the x87 stack: divided when `divide`,
/// else multiplied.
fn scaled_by_divisor(e: &Engine, dot: u32, divide: bool) -> f64 {
    let divisor = e.mem.f64(VELOCITY_DIVISOR);
    let value = x87_load32(dot);
    if divide {
        x87_div(value, divisor)
    } else {
        x87_mul(value, divisor)
    }
}

// Translated from 00db49b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk4dGskCalcPointVelocityAssumingAngularVelError`: the error of the point velocity along
/// `direction`. `axes` arrives in `ECX` (two vectors, at `+0` and `+0x10`) and `second_motion`
/// in `EDX`; `first_motion` and the two motion records hold a start position at `+0x40` with
/// its time in the fourth lane, an end position at `+0x50` with a scale in its fourth lane.
/// `time` is a float. With `d_n = point - position_n(time)` the result is
/// `-(w . direction) + (axes[0] x d_0) . direction / 1.2` (divided when positive, else
/// multiplied) minus the same term with `axes[1]` and the second record (divided when
/// negative, else multiplied); the answer is a float in `ST0`.
pub fn calc_point_velocity_assuming_angular_velocity_error(
    e: &mut Engine,
    axes: Ptr,
    second_motion: Ptr,
    first_motion: Ptr,
    point: Ptr,
    direction: Ptr,
    angular_velocity: Ptr,
    time: u32,
) -> f32 {
    let time = x87_load32(time);
    let direction = load_lanes(e, direction.addr());
    let mask = load_lanes(e, SIGN_MASK);
    let angular = load_lanes(e, angular_velocity.addr());
    let negated = [0, 1, 2, 3].map(|lane| angular[lane] ^ mask[lane]);
    let angular_term = lane_sum(lanes_op(negated, direction, mul));
    let point = load_lanes(e, point.addr());
    let axes = axes.addr();

    let fraction = motion_fraction(e, first_motion.addr(), time);
    let position = blended_position(e, first_motion.addr(), fraction);
    let offset = lanes_op(point, position, sub);
    let dot = cross_dot(load_lanes(e, axes), offset, direction);
    let value = x87_load32(dot);
    let divide = value > 0.0 || value.is_nan();
    let scaled = scaled_by_divisor(e, dot, divide);
    let partial = x87_store32(x87_add(scaled, x87_load32(angular_term)));

    let fraction = motion_fraction(e, second_motion.addr(), time);
    let position = blended_position(e, second_motion.addr(), fraction);
    let offset = lanes_op(point, position, sub);
    let dot = cross_dot(load_lanes(e, axes + 0x10), offset, direction);
    let value = x87_load32(dot);
    let divide = value < 0.0 || value.is_nan();
    let scaled = scaled_by_divisor(e, dot, divide);
    f32::from_bits(x87_store32(x87_sub(x87_load32(partial), scaled)))
}

// Translated from 00db4b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk4dGskCollideInitMatrices`: fills the `0x130`-byte work area `out` for the interpolation
/// `fraction` (a float) of the time step in the collision `state` (the two bodies at `+0` and
/// `+4`, the time record at `+0x40`): the two interpolated transforms at `out` and `out+0x40`,
/// their relative transform at `out+0x80`, the plane normal at `out+0xc0` (its fourth lane the
/// negated offset at `state+0x14`) and its position at `out+0xd0`, the two transformed rows
/// at `out+0xf0` and `out+0x100`, a body vector copy at `out+0x110`, and two transformed
/// positions at `out+0x120` and `out+0xe0`.
pub fn collide_init_matrices(e: &mut Engine, state: Ptr, fraction: u32, out: Ptr) {
    let state = state.addr();
    let out = out.addr();
    let times = e.mem.u32(state + 0x40);
    let first_body = e.mem.u32(state);
    let span_value = x87_mul(x87_load32(e.mem.u32(times + 8)), x87_load32(fraction));
    let span = x87_quiet_bits(x87_store32(span_value));
    let start = x87_quiet_bits(e.mem.u32(times));
    e.call(
        INTERPOLATE_TRANSFORM,
        &args![first_body + 0x40, start, span, out],
    );
    let second_body = e.mem.u32(state + 4);
    e.call(
        INTERPOLATE_TRANSFORM,
        &args![second_body + 0x40, start, span, out + 0x40],
    );
    e.call(SET_MUL_INVERSE_MUL, &args![out + 0x80, out, out + 0x40]);
    e.call(
        SET_ROTATED_DIR,
        &args![out + 0xc0, out + 0x80, state + 0x20],
    );
    e.call(
        SET_TRANSFORMED_POS,
        &args![out + 0xd0, out + 0x80, state + 0x30],
    );
    let offset = x87_quiet_bits(e.mem.u32(state + 0x14)) ^ 0x8000_0000;
    e.mem.set_u32(out + 0xcc, offset);
    let first_body = e.mem.u32(state);
    copy_vector(e, first_body + 0x80, out + 0x110);

    // The three rows transposed into columns.
    let rows = [0, 0x10, 0x20].map(|row| load_lanes(e, out + row));
    let columns = transpose_rows(rows);
    for row in 0..2 {
        let v = load_lanes(e, state + 0x60 + 0x10 * row);
        let spread = |lane: usize| [v[lane]; 4];
        let from_y = lanes_op(spread(1), columns[1], mul);
        let from_x = lanes_op(spread(0), columns[0], mul);
        let sum = lanes_op(from_y, from_x, add);
        let from_z = lanes_op(spread(2), columns[2], mul);
        store_lanes(e, out + 0xf0 + 0x10 * row, lanes_op(sum, from_z, add));
    }
    let second_body = e.mem.u32(state + 4);
    e.call(
        SET_TRANSFORMED_POS,
        &args![out + 0x120, out + 0x80, second_body + 0x80],
    );
    e.call(
        TRANSFORM_UNNAMED_004B4AB0,
        &args![out + 0xe0, out, state + 0x50],
    );
}

// Translated from 00db4ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk4dGskCollidePointWithPlaneFinal`: refines, by up to ten interpolation steps between the
/// fractions `a` and `b` (whose signed distances to the plane are `c` and `d`), the fraction
/// where `point` reaches the plane distance in `plane[0]`; the tolerance is a quarter of
/// `plane[4]`. The fraction is clamped to `0.1..0.9` on every even step. If it is below the
/// time of impact in `best[0]`, that and the point (at `best+0x10`) are replaced. Answers
/// `best[0]` as a float. The word after `d` is not used.
pub fn collide_point_with_plane_final(
    e: &mut Engine,
    state: Ptr,
    plane: Ptr,
    point: Ptr,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    _unused_1: u32,
    best: Ptr,
) -> f32 {
    let plane = plane.addr();
    let best = best.addr();
    let (mut a, mut b, mut c, mut d) = (a, b, c, d);
    let mut fraction = x87_quiet_bits(a);
    let tolerance = x87_store32(x87_mul(
        x87_load32(e.mem.u32(plane + 0x10)),
        e.mem.f64(QUARTER),
    ));
    for step in 0..10u32 {
        let target = x87_load32(e.mem.u32(plane));
        let near = x87_load32(c);
        if target >= near {
            break;
        }
        let span = x87_store32(x87_sub(x87_load32(d), near));
        if -x87_load32(tolerance) < x87_load32(span) {
            break;
        }
        fraction = x87_store32(x87_div(x87_sub(target, near), x87_load32(span)));
        if step % 2 == 0 {
            let above_lower = x87_load32(fraction) > e.mem.f64(LOWER_FRACTION);
            if !above_lower {
                fraction = x87_quiet_bits(e.mem.u32(LOWER_FRACTION_FLOAT));
            }
            let below_upper = x87_load32(fraction) < e.mem.f64(UPPER_FRACTION);
            if !below_upper {
                fraction = x87_quiet_bits(e.mem.u32(UPPER_FRACTION_FLOAT));
            }
        }
        let weight = x87_load32(fraction);
        let from_start = x87_mul(x87_sub(1.0, weight), x87_load32(a));
        let from_end = x87_mul(weight, x87_load32(b));
        fraction = x87_store32(x87_add(from_start, from_end));

        let distance = e.with_stack(0x130, |e, work| {
            let work = work.addr();
            collide_init_matrices(e, state, x87_quiet_bits(fraction), Ptr::new(work));
            let position = load_lanes(e, work + 0xd0);
            let normal = load_lanes(e, work + 0xc0);
            let difference = lanes_op(load_lanes(e, point.addr()), position, sub);
            let product = lanes_op(difference, normal, mul);
            let high = sse(product[2], normal[3], add);
            let low = sse(product[1], product[0], add);
            sse(high, low, add)
        });
        let error = x87_store32(x87_sub(x87_load32(distance), x87_load32(e.mem.u32(plane))));
        if x87_load32(tolerance) > x87_load32(error).abs() {
            break;
        }
        let reached = x87_load32(e.mem.u32(plane));
        let below = reached > x87_load32(distance);
        if below {
            b = x87_quiet_bits(fraction);
            d = x87_quiet_bits(distance);
        } else {
            a = x87_quiet_bits(fraction);
            c = x87_quiet_bits(distance);
        }
    }
    let current = x87_load32(e.mem.u32(best));
    if current > x87_load32(fraction) {
        let replaced = x87_quiet_bits(fraction);
        e.mem.set_u32(best, replaced);
        copy_vector(e, point.addr(), best + 0x10);
        f32::from_bits(replaced)
    } else {
        f32::from_bits(x87_store32(current))
    }
}

// Translated from 00db4e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk4dGskCollidePointsWithPlane`: sorts the points of the collision `state` (`state[2]` an
/// array of 16-byte points, `state[3]` their number) that can reach the plane `plane` (three
/// floats of limits at `+0`, `+4`, `+8`) to the front and, from the first interpolation
/// fraction `state[0x80]` on, narrows the time of impact stored with its point in `out`
/// (`out[0]`, the point at `out+0x10`): a point whose distance is at most the plane limit
/// plus the margin (`state[0x11]` times the remaining time span) is tested further; one that
/// is on the near side of the plane is handed to
/// [`collide_point_with_plane_final`] (or, in the first round, written to `out` as is), one
/// beyond it is moved to the front part of the array with its distance in the fourth lane and
/// the next fraction is the least time the point can reach the limit. Rounds repeat while
/// points were moved and the next fraction does not pass the best time. The square roots go
/// through the x87 square root (`00ec6040`).
pub fn collide_points_with_plane(e: &mut Engine, state: Ptr, plane: Ptr, out: Ptr) {
    e.with_stack(0x130, |e, work| {
        let work = work.addr();
        let state_ptr = state;
        let state = state.addr();
        let plane = plane.addr();
        let out = out.addr();
        // The slots of the frame: the best time, the current and the previous fraction, the
        // number of points moved this round.
        let mut best = 1.0f32.to_bits();
        let mut start = x87_quiet_bits(e.mem.u32(state + 0x80));
        let mut previous = start;
        let mut kept = e.mem.u32(state + 0xc);
        if kept == 0 {
            return;
        }
        loop {
            collide_init_matrices(e, state_ptr, x87_quiet_bits(start), Ptr::new(work));
            let best_value = x87_load32(best);
            let start_value = x87_load32(start);
            let span = x87_store32(x87_sub(best_value, start_value));
            let span_value = x87_load32(span);
            let mut counter = kept.wrapping_sub(1) as i32;
            kept = 0;
            let points = e.mem.u32(state + 8);
            let mut read = points;
            let mut write = points;
            let margin = x87_store32(x87_mul(x87_load32(e.mem.u32(state + 0x44)), span_value));
            let mut candidate = x87_store32(best_value);
            while counter >= 0 {
                'point: {
                    let point = load_lanes(e, read);
                    let position = load_lanes(e, work + 0xd0);
                    let normal = load_lanes(e, work + 0xc0);
                    let difference = lanes_op(point, position, sub);
                    let product = lanes_op(difference, normal, mul);
                    let high = sse(product[2], normal[3], add);
                    let low = sse(product[1], product[0], add);
                    let distance = sse(high, low, add);
                    let limit = e.mem.u32(plane);
                    let reach = x87_add(x87_load32(limit), x87_load32(margin));
                    let distance_value = x87_load32(distance);
                    if reach < distance_value {
                        break 'point;
                    }
                    let excess = x87_store32(x87_sub(distance_value, x87_load32(limit)));
                    let from_first = lanes_op(point, load_lanes(e, work + 0x110), sub);
                    let from_second = lanes_op(point, load_lanes(e, work + 0x120), sub);
                    let first = lanes_op(
                        cross(load_lanes(e, work + 0xf0), from_first),
                        load_lanes(e, work + 0xe0),
                        sub,
                    );
                    let second = cross(load_lanes(e, work + 0x100), from_second);
                    let turning = lane_sum(lanes_op(lanes_op(first, second, sub), normal, mul));
                    let rate = x87_mul(x87_load32(turning), span_value);
                    let growth = x87_store32(x87_add(x87_load32(excess), rate));
                    let allowed = x87_mul(x87_mul(span_value, e.mem.f64(HALF)), span_value);
                    let allowed =
                        x87_store32(x87_mul(allowed, x87_load32(e.mem.u32(state + 0x18))));
                    if x87_load32(allowed) < x87_load32(growth) {
                        break 'point;
                    }
                    if x87_load32(e.mem.u32(plane)) > distance_value {
                        let lower = x87_load32(start);
                        if x87_load32(e.mem.u32(state + 0x80)) == lower {
                            let first_time = x87_store32(lower);
                            e.mem.set_u32(out, first_time);
                            best = first_time;
                            copy_vector(e, read, out + 0x10);
                        } else {
                            let point_w = x87_quiet_bits(e.mem.u32(read + 0xc));
                            let answer = collide_point_with_plane_final(
                                e,
                                state_ptr,
                                Ptr::new(plane),
                                Ptr::new(read),
                                x87_quiet_bits(previous),
                                x87_store32(lower),
                                point_w,
                                x87_store32(distance_value),
                                x87_quiet_bits(turning),
                                Ptr::new(out),
                            );
                            best = answer.to_bits();
                        }
                        break 'point;
                    }
                    // The point is moved to the front part of the array.
                    let moved = load_lanes(e, read);
                    let front = load_lanes(e, write);
                    store_lanes(e, read, front);
                    store_lanes(e, write, moved);
                    e.mem.set_u32(write + 0xc, x87_store32(distance_value));
                    kept += 1;
                    write += 0x10;
                    let beyond =
                        x87_store32(x87_sub(distance_value, x87_load32(e.mem.u32(plane + 4))));
                    let two = e.mem.f64(TWO);
                    let scaled = x87_store32(x87_mul(
                        x87_mul(x87_load32(beyond), two),
                        x87_load32(e.mem.u32(state + 0x1c)),
                    ));
                    if x87_load32(scaled) > x87_mul(span_value, span_value) {
                        break 'point;
                    }
                    let gap = x87_store32(x87_sub(
                        x87_load32(e.mem.u32(plane)),
                        x87_load32(e.mem.u32(plane + 4)),
                    ));
                    let wide = x87_store32(x87_mul(
                        x87_mul(two, x87_load32(gap)),
                        x87_load32(e.mem.u32(state + 0x1c)),
                    ));
                    let root = e.call(X87_SQRT, &args![x87_load32(scaled)]).f64();
                    let mut reach_time = x87_quiet_bits(x87_store32(root));
                    let turning_value = x87_load32(turning);
                    let rate_factor = x87_load32(e.mem.u32(state + 0x1c));
                    if turning_value < 0.0 || turning_value.is_nan() {
                        let negated = x87_store32(x87_mul(-turning_value, rate_factor));
                        if x87_load32(reach_time) < x87_load32(negated) {
                            reach_time = x87_quiet_bits(negated);
                        }
                    } else {
                        let doubled = x87_mul(turning_value, rate_factor);
                        let doubled = x87_add(doubled, doubled);
                        reach_time = x87_store32(x87_add(doubled, x87_load32(reach_time)));
                    }
                    if span_value < x87_load32(reach_time) {
                        break 'point;
                    }
                    let wide_root = e.call(X87_SQRT, &args![x87_load32(wide)]).f64();
                    let wide_time = x87_store32(wide_root);
                    let total = x87_store32(x87_add(x87_load32(wide_time), x87_load32(reach_time)));
                    let third = x87_quiet_bits(e.mem.u32(plane + 8));
                    reach_time = if x87_load32(third) < x87_load32(total) {
                        total
                    } else {
                        third
                    };
                    let next = x87_store32(x87_add(x87_load32(reach_time), x87_load32(start)));
                    let later = x87_load32(next) > x87_load32(candidate);
                    if !later {
                        candidate = x87_quiet_bits(next);
                    }
                }
                read += 0x10;
                counter = counter.wrapping_sub(1);
            }
            let best_value = x87_load32(best);
            let start_value = x87_load32(start);
            if best_value < start_value {
                return;
            }
            let candidate_value = x87_load32(candidate);
            let continues =
                candidate_value != start_value || best_value > start_value || best_value.is_nan();
            if !continues {
                return;
            }
            previous = x87_store32(start_value);
            start = x87_store32(candidate_value);
            if kept == 0 {
                return;
            }
        }
    });
}

// Translated from 00db52c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk4dGskCalcSeparatingNormal`: the separating normal and a point for the shape pair.
/// Register convention: `gsk` is `EDI` (the `hkpGsk`), `transform_a` is `ESI` and
/// `transform_b` is `EAX`; `shape_a`, `shape_b`, `out_normal` and `out_second` are the stack
/// words. Takes the closest feature `A` (a direction, its fourth lane a distance) between
/// the shapes in the frame of `transform_a` relative to `transform_b`, writes the direction
/// turned by the rows of `transform_a` to `out_normal` (its fourth lane: the distance minus the
/// two radii at `+0x10` of the shapes), and writes the point (from the `hkpGsk` simplex by its
/// two type words, less the distance times `A` and the relative translation) turned by the
/// transposed relative rotation to `out_second`.
pub fn calc_separating_normal(
    e: &mut Engine,
    gsk: Ptr,
    transform_a: Ptr,
    transform_b: Ptr,
    shape_a: Ptr,
    shape_b: Ptr,
    out_normal: Ptr,
    out_second: Ptr,
) {
    e.with_stack(0x50, |e, frame| {
        let relative = frame.addr();
        let closest = relative + 0x40;
        e.call(
            SET_MUL_INVERSE_MUL,
            &args![relative, transform_a, transform_b],
        );
        e.call(
            GET_CLOSEST_FEATURE,
            &args![gsk, shape_a, shape_b, relative, closest],
        );
        let a = load_lanes(e, closest);
        let spread = |lane: usize| [a[lane]; 4];
        let rows = [0, 0x10, 0x20].map(|row| load_lanes(e, transform_a.addr() + row));
        let toward_y = lanes_op(spread(1), rows[1], mul);
        let toward_x = lanes_op(spread(0), rows[0], mul);
        let partial = lanes_op(toward_y, toward_x, add);
        let toward_z = lanes_op(spread(2), rows[2], mul);
        store_lanes(e, out_normal.addr(), lanes_op(partial, toward_z, add));

        let gsk = gsk.addr();
        let along = lanes_op(spread(3), a, mul);
        let feature = if e.mem.u32(gsk) == 1 {
            load_lanes(e, gsk + 0x20)
        } else if e.mem.u32(gsk + 4) == 1 {
            lanes_op(along, load_lanes(e, gsk + 0xa0), add)
        } else {
            load_lanes(e, gsk + 0x130)
        };
        let distance = x87_load32(a[3]);
        let rotation = [0, 0x10, 0x20].map(|row| load_lanes(e, relative + row));
        let translation = load_lanes(e, relative + 0x30);
        let moved = lanes_op(lanes_op(feature, along, sub), translation, sub);
        let columns = transpose_rows(rotation);
        let spread = |lane: usize| [moved[lane]; 4];
        let toward_y = lanes_op(spread(1), columns[1], mul);
        let toward_x = lanes_op(spread(0), columns[0], mul);
        let toward_z = lanes_op(columns[2], spread(2), mul);
        let partial = lanes_op(toward_y, toward_x, add);
        store_lanes(e, out_second.addr(), lanes_op(partial, toward_z, add));
        let gap = x87_sub(distance, x87_load32(e.mem.u32(shape_a.addr() + 0x10)));
        let gap = x87_sub(gap, x87_load32(e.mem.u32(shape_b.addr() + 0x10)));
        e.mem.set_u32(out_normal.addr() + 0xc, x87_store32(gap));
    });
}

// Translated from 00db53f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the collision `state` (`ESI`) from the description `input` (`EAX`; no name known):
/// `input[0]` and `input[1]` point to two records whose first word is an object (with the
/// virtual slots `0x2c` and `0x30` for the first) and whose third word is a pointer to
/// a body-like block. Stores the two block pointers at `state[0]` and `state[1]`, the sum of
/// the two objects' floats at `+0x10` at `state+0x14`, the vector at `input+0x70` at
/// `state+0x50`, `input[3] + 0x50` at `state+0x40`, the first object's slot `0x2c` answer
/// at `state+0xc` and `state+0x10`, and a `0x80`-aligned allocation of that many 16-byte
/// elements (the allocator's cursor lives in the first word of the object `00c85750`
/// answers) at `state+8`, which the object's slot `0x30` is then given. Finally
/// `state+0x18` is `b[0xa0] * s[0x7c]^2 + a[0xa0] * s[0x6c]^2` (`a`, `b` the two block
/// pointers, `s` the state) and `state+0x1c` its reciprocal plus the epsilon double.
pub fn fn_00db53f0(e: &mut Engine, input: Ptr, state: Ptr) {
    let input = input.addr();
    let state = state.addr();
    let first = e.mem.u32(input);
    let second = e.mem.u32(input + 4);
    let object = e.mem.u32(first);
    let first_block = e.mem.u32(first + 8);
    let other = e.mem.u32(second);
    let second_block = e.mem.u32(second + 8);
    e.mem.set_u32(state, first_block);
    e.mem.set_u32(state + 4, second_block);
    let radii = x87_add(
        x87_load32(e.mem.u32(other + 0x10)),
        x87_load32(e.mem.u32(object + 0x10)),
    );
    e.mem.set_u32(state + 0x14, x87_store32(radii));
    copy_vector(e, input + 0x70, state + 0x50);
    let base = e.mem.u32(input + 0xc).wrapping_add(0x50);
    e.mem.set_u32(state + 0x40, base);
    let count = e.vcall(object, 0x2c, &args![]).u32();
    e.mem.set_u32(state + 0x10, count);
    e.mem.set_u32(state + 0xc, count);
    let allocator = e.call(MEMORY_ROUTER, &args![]).u32();
    let cursor = e.mem.u32(allocator);
    let size = (count.wrapping_mul(16).wrapping_add(0x7f) & 0xffff_ff80).wrapping_add(cursor);
    e.mem.set_u32(allocator, size);
    e.mem.set_u32(state + 8, cursor);
    e.vcall(object, 0x30, &args![cursor]);

    let second_term = x87_mul(
        x87_mul(
            x87_load32(e.mem.u32(second_block + 0xa0)),
            x87_load32(e.mem.u32(state + 0x7c)),
        ),
        x87_load32(e.mem.u32(state + 0x7c)),
    );
    let second_term = x87_load32(x87_store32(second_term));
    let first_term = x87_store32(x87_mul(
        x87_mul(
            x87_load32(e.mem.u32(first_block + 0xa0)),
            x87_load32(e.mem.u32(state + 0x6c)),
        ),
        x87_load32(e.mem.u32(state + 0x6c)),
    ));
    let total = x87_store32(x87_add(second_term, x87_load32(first_term)));
    e.mem.set_u32(state + 0x18, total);
    let divisor = x87_add(x87_load32(total), e.mem.f64(EPSILON));
    e.mem
        .set_u32(state + 0x1c, x87_store32(x87_div(1.0, divisor)));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00db_49b0,
            calc_point_velocity_assuming_angular_velocity_error(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, u32) -> f32
        ),
        entry!(0x00db_4b50, collide_init_matrices(Ptr, u32, Ptr)),
        entry!(0x00db_4e40, collide_points_with_plane(Ptr, Ptr, Ptr)),
        entry!(
            0x00db_52c0,
            calc_separating_normal(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00db_53f0, fn_00db53f0(Ptr, Ptr)),
        entry!(
            0x00db_4ca0,
            collide_point_with_plane_final(Ptr, Ptr, Ptr, u32, u32, u32, u32, u32, Ptr) -> f32
        ),
    ]
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;

    /// An engine with the exe's data range mapped and the constants this unit reads.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        for lane in 0..4 {
            e.mem.set_f32(VECTOR_ONES + 4 * lane, 1.0);
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        e.mem.set_f64(VELOCITY_DIVISOR, f64::from(1.2f32));
        e.mem.set_f64(QUARTER, 0.25);
        e.mem.set_f64(HALF, 0.5);
        e.mem.set_f64(TWO, 2.0);
        e.mem.set_f64(LOWER_FRACTION, f64::from(0.1f32));
        e.mem.set_f64(UPPER_FRACTION, f64::from(0.9f32));
        e.mem.set_f32(LOWER_FRACTION_FLOAT, 0.1);
        e.mem.set_f32(UPPER_FRACTION_FLOAT, 0.9);
        e
    }

    /// Writes four floats at `at`.
    fn put(e: &mut Engine, at: u32, values: [f32; 4]) {
        for (lane, value) in values.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, value);
        }
    }

    /// A vector of four floats in new memory.
    fn vector(e: &mut Engine, values: [f32; 4]) -> u32 {
        let at = e.mem.alloc(0x10);
        put(e, at, values);
        at
    }

    /// A motion record: a start position and time, an end position and scale.
    fn motion(e: &mut Engine, start: [f32; 4], end: [f32; 4]) -> u32 {
        let record = e.mem.alloc(0x60);
        put(e, record + 0x40, start);
        put(e, record + 0x50, end);
        record
    }

    /// The point velocity error for the fixed scene: the motions, the axes and the point.
    fn velocity_error(direction: [f32; 4], time: u32) -> f32 {
        let mut e = rig();
        let first = motion(&mut e, [0.0, 0.0, 0.0, 1.0], [4.0, 0.0, 0.0, 0.5]);
        let second = motion(&mut e, [0.0, 0.0, 0.0, 0.0], [0.0, 4.0, 0.0, 0.25]);
        let axes = e.mem.alloc(0x20);
        put(&mut e, axes, [0.0, 1.0, 0.0, 0.0]);
        put(&mut e, axes + 0x10, [1.0, 0.0, 0.0, 0.0]);
        let point = vector(&mut e, [1.0, 1.0, 1.0, 0.0]);
        let direction = vector(&mut e, direction);
        let angular = vector(&mut e, [1.0, 1.0, 1.0, 0.0]);
        e.call(
            0x00db_49b0,
            &args![axes, second, first, point, direction, angular, time],
        )
        .f32()
    }

    #[test]
    fn point_velocity_error_divides_a_positive_term_and_multiplies_a_negative_one() {
        // d_0 = (-1, 1, 1): axes[0] x d_0 = (1, 0, 1); d_1 = (1, -1, 1): axes[1] x d_1 =
        // (0, -1, -1). Direction (1, 2, 3): angular term -6, first dot 4 (divided by 1.2),
        // second dot -5 (divided by 1.2): -6 + 4 / 1.2 + 5 / 1.2 = 1.5.
        let result = velocity_error([1.0, 2.0, 3.0, 0.0], 2.0f32.to_bits());
        assert!((result - 1.5).abs() < 1e-4, "{result}");
        // Direction (-1, -2, -3): angular term 6, first dot -4 (multiplied: -4.8), second
        // dot 5 (multiplied: 6): 6 - 4.8 - 6 = -4.8.
        let result = velocity_error([-1.0, -2.0, -3.0, 0.0], 2.0f32.to_bits());
        assert!((result + 4.8).abs() < 1e-4, "{result}");
    }

    #[test]
    fn point_velocity_error_of_a_nan_time_is_nan() {
        let result = velocity_error([1.0, 2.0, 3.0, 0.0], 0x7fc0_0000);
        assert!(result.is_nan());
    }

    /// A collision state: two bodies (vector at `+0x80`), the times (`+0` start, `+8` span),
    /// the plane data, and two rows at `+0x60`.
    struct State {
        state: u32,
        first_body: u32,
        second_body: u32,
        times: u32,
    }

    fn collision_state(e: &mut Engine) -> State {
        let state = e.mem.alloc(0x80);
        let first_body = e.mem.alloc(0x90);
        let second_body = e.mem.alloc(0x90);
        let times = e.mem.alloc(0x10);
        e.mem.set_u32(state, first_body);
        e.mem.set_u32(state + 4, second_body);
        e.mem.set_u32(state + 0x40, times);
        put(e, times, [0.25, 0.0, 4.0, 0.0]);
        put(e, state + 0x14, [3.5, 0.0, 0.0, 0.0]);
        put(e, state + 0x60, [1.0, 2.0, 3.0, 0.0]);
        put(e, state + 0x70, [0.0, 1.0, 0.0, 0.0]);
        put(e, first_body + 0x80, [9.0, 8.0, 7.0, 6.0]);
        State {
            state,
            first_body,
            second_body,
            times,
        }
    }

    /// Doubles for the five callees outside the unit (they write nothing).
    fn plain_callees(e: &mut Engine) {
        e.call_log = Some(Vec::new());
        for address in [
            INTERPOLATE_TRANSFORM,
            SET_MUL_INVERSE_MUL,
            SET_ROTATED_DIR,
            SET_TRANSFORMED_POS,
            TRANSFORM_UNNAMED_004B4AB0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
    }

    /// The argument lists of the logged calls to `address`.
    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(callee, _)| *callee == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn init_matrices_interpolates_both_bodies_and_transposes_the_rows() {
        let mut e = rig();
        plain_callees(&mut e);
        let s = collision_state(&mut e);
        let out = e.mem.alloc(0x130);
        put(&mut e, out, [1.0, 2.0, 3.0, 4.0]);
        put(&mut e, out + 0x10, [5.0, 6.0, 7.0, 8.0]);
        put(&mut e, out + 0x20, [9.0, 10.0, 11.0, 12.0]);
        e.call(0x00db_4b50, &args![s.state, 0.5f32.to_bits(), out]);

        let span = 2.0f32.to_bits();
        let start = 0.25f32.to_bits();
        assert_eq!(
            calls_to(&e, INTERPOLATE_TRANSFORM),
            vec![
                vec![s.first_body + 0x40, start, span, out],
                vec![s.second_body + 0x40, start, span, out + 0x40],
            ]
        );
        assert_eq!(
            calls_to(&e, SET_MUL_INVERSE_MUL),
            vec![vec![out + 0x80, out, out + 0x40]]
        );
        assert_eq!(
            calls_to(&e, SET_ROTATED_DIR),
            vec![vec![out + 0xc0, out + 0x80, s.state + 0x20]]
        );
        assert_eq!(
            calls_to(&e, SET_TRANSFORMED_POS),
            vec![
                vec![out + 0xd0, out + 0x80, s.state + 0x30],
                vec![out + 0x120, out + 0x80, s.second_body + 0x80],
            ]
        );
        assert_eq!(
            calls_to(&e, TRANSFORM_UNNAMED_004B4AB0),
            vec![vec![out + 0xe0, out, s.state + 0x50]]
        );
        assert_eq!(e.mem.f32(out + 0xcc), -3.5);
        let copied = load_lanes(&e, out + 0x110);
        assert_eq!(copied, [9.0f32, 8.0, 7.0, 6.0].map(f32::to_bits));
        // Columns (1, 5, 9, 10), (2, 6, 10, 12), (3, 7, 11, 12): the first row (1, 2, 3) gives
        // 2 * c1 + 1 * c0 + 3 * c2, the second (0, 1, 0) the column c1.
        let first_row = [14.0f32, 38.0, 62.0, 70.0].map(f32::to_bits);
        assert_eq!(load_lanes(&e, out + 0xf0), first_row);
        let second_row = [2.0f32, 6.0, 10.0, 12.0].map(f32::to_bits);
        assert_eq!(load_lanes(&e, out + 0x100), second_row);
    }

    #[test]
    fn init_matrices_passes_the_time_and_step_quieted() {
        let mut e = rig();
        plain_callees(&mut e);
        let s = collision_state(&mut e);
        let out = e.mem.alloc(0x130);
        e.mem.set_u32(s.times, 0x7f80_0001);
        e.call(0x00db_4b50, &args![s.state, 0x3f80_0000u32, out]);
        let calls = calls_to(&e, INTERPOLATE_TRANSFORM);
        assert_eq!(calls[0][1], 0x7fc0_0001);
        assert_eq!(calls[1][1], 0x7fc0_0001);
        assert_eq!(calls[0][2], 4.0f32.to_bits());
    }

    /// The doubles that make the plane distance of the point at the origin `-g(f)`, with
    /// `g(f) = f` (`squared` false) or `f * f`: the interpolation writes its span (`4 * f`) over 4
    /// into the first word of the work area, the position double stores `g(f)` and the normal is `(1, 0, 0)`.
    fn distance_scene(squared: bool) -> (Engine, State) {
        let mut e = rig();
        e.call_log = Some(Vec::new());
        e.register(INTERPOLATE_TRANSFORM, |e, a| {
            e.mem.set_f32(a[3], f32::from_bits(a[2]) / 4.0);
            Ret::default()
        });
        e.register(SET_MUL_INVERSE_MUL, |_, _| Ret::default());
        e.register(SET_ROTATED_DIR, |e, a| {
            put(e, a[0], [1.0, 0.0, 0.0, 0.0]);
            Ret::default()
        });
        if squared {
            e.register(SET_TRANSFORMED_POS, |e, a| {
                let f = e.mem.f32(a[1] - 0x80);
                put(e, a[0], [f * f, 0.0, 0.0, 0.0]);
                Ret::default()
            });
        } else {
            e.register(SET_TRANSFORMED_POS, |e, a| {
                let f = e.mem.f32(a[1] - 0x80);
                put(e, a[0], [f, 0.0, 0.0, 0.0]);
                Ret::default()
            });
        }
        e.register(TRANSFORM_UNNAMED_004B4AB0, |_, _| Ret::default());
        let s = collision_state(&mut e);
        put(&mut e, s.state + 0x14, [0.0, 0.0, 0.0, 0.0]);
        (e, s)
    }

    /// Calls the refinement from fraction 0 (distance 0) to 1 (distance -1) for a plane at
    /// `target`, whose tolerance is a quarter of `size`, with the best time `best` and a point
    /// vector there that is 7 in every lane. Answers the result and the best-time record.
    fn refine(e: &mut Engine, s: &State, size: f32, target: f32, best: f32) -> (f32, u32) {
        let plane = e.mem.alloc(0x20);
        put(e, plane, [target, 0.0, 0.0, 0.0]);
        e.mem.set_f32(plane + 0x10, size);
        let point = vector(e, [0.0, 0.0, 0.0, 1.0]);
        let result = e.mem.alloc(0x20);
        e.mem.set_f32(result, best);
        put(e, result + 0x10, [7.0; 4]);
        let answer = e
            .call(
                0x00db_4ca0,
                &args![
                    s.state,
                    plane,
                    point,
                    0.0f32.to_bits(),
                    1.0f32.to_bits(),
                    0.0f32.to_bits(),
                    (-1.0f32).to_bits(),
                    0xdead_beefu32,
                    result
                ],
            )
            .f32();
        (answer, result)
    }

    #[test]
    fn final_refinement_stops_when_the_fraction_is_inside_the_tolerance() {
        let (mut e, s) = distance_scene(false);
        let (answer, result) = refine(&mut e, &s, 0.04, -0.3, 10.0);
        // Distance -f: the first secant step lands on f = 0.3 exactly.
        assert!((answer - 0.3).abs() < 1e-6, "{answer}");
        assert_eq!(e.mem.f32(result), answer);
        assert_eq!(
            load_lanes(&e, result + 0x10),
            [0.0f32, 0.0, 0.0, 1.0].map(f32::to_bits)
        );
        assert_eq!(calls_to(&e, INTERPOLATE_TRANSFORM).len(), 2);
    }

    #[test]
    fn final_refinement_keeps_a_smaller_time_of_impact() {
        let (mut e, s) = distance_scene(false);
        let (answer, result) = refine(&mut e, &s, 0.04, -0.3, 0.1);
        assert_eq!(answer, 0.1);
        assert_eq!(e.mem.f32(result), 0.1);
        assert_eq!(load_lanes(&e, result + 0x10), [7.0f32; 4].map(f32::to_bits));
    }

    #[test]
    fn final_refinement_without_a_tolerance_takes_ten_steps() {
        let (mut e, s) = distance_scene(true);
        let (answer, result) = refine(&mut e, &s, 0.0, -0.5, 10.0);
        assert_eq!(calls_to(&e, INTERPOLATE_TRANSFORM).len(), 20);
        // The distance is -f * f, so the plane at -0.5 is reached at f = 0.7071.
        assert!(
            (answer - std::f32::consts::FRAC_1_SQRT_2).abs() < 0.05,
            "{answer}"
        );
        assert_eq!(e.mem.f32(result), answer);
    }

    #[test]
    fn final_refinement_returns_at_once_when_the_target_is_not_below_the_start() {
        let (mut e, s) = distance_scene(false);
        // Target 0 is not below the start distance 0.
        let (answer, result) = refine(&mut e, &s, 0.04, 0.0, 10.0);
        assert!(calls_to(&e, INTERPOLATE_TRANSFORM).is_empty());
        // The start fraction 0 is below the best 10: stored.
        assert_eq!(answer, 0.0);
        assert_eq!(e.mem.f32(result), 0.0);
    }

    #[test]
    fn final_refinement_stops_when_the_distances_do_not_fall() {
        let (mut e, s) = distance_scene(false);
        let plane = e.mem.alloc(0x20);
        put(&mut e, plane, [-0.3, 0.0, 0.0, 0.0]);
        e.mem.set_f32(plane + 0x10, 0.04);
        let point = vector(&mut e, [0.0, 0.0, 0.0, 1.0]);
        let result = e.mem.alloc(0x20);
        e.mem.set_f32(result, 10.0);
        // The end distance +1 is above the start distance 0: the step is not negative.
        e.call(
            0x00db_4ca0,
            &args![
                s.state,
                plane,
                point,
                0.0f32.to_bits(),
                1.0f32.to_bits(),
                0.0f32.to_bits(),
                1.0f32.to_bits(),
                0u32,
                result
            ],
        );
        assert!(calls_to(&e, INTERPOLATE_TRANSFORM).is_empty());
    }

    /// The relative transform the double for `setMulInverseMul` writes: rows
    /// `(1,2,3,4)`, `(5,6,7,8)`, `(9,10,11,12)` and the translation `(0.5,1,1.5,2)`.
    fn write_relative(e: &mut Engine, a: &[u32]) -> Ret {
        put(e, a[0], [1.0, 2.0, 3.0, 4.0]);
        put(e, a[0] + 0x10, [5.0, 6.0, 7.0, 8.0]);
        put(e, a[0] + 0x20, [9.0, 10.0, 11.0, 12.0]);
        put(e, a[0] + 0x30, [0.5, 1.0, 1.5, 2.0]);
        Ret::default()
    }

    /// The closest feature the double for `getClosestFeature` writes: `(0, 1, 0)`, distance 2.5.
    fn write_feature(e: &mut Engine, a: &[u32]) -> Ret {
        put(e, a[4], [0.0, 1.0, 0.0, 2.5]);
        Ret::default()
    }

    /// Runs the separating normal with the simplex type words `(first, second)`; answers the
    /// two output vectors.
    fn separating(first: u32, second: u32) -> (Lanes, Lanes) {
        let mut e = rig();
        e.call_log = Some(Vec::new());
        e.register(SET_MUL_INVERSE_MUL, write_relative);
        e.register(GET_CLOSEST_FEATURE, write_feature);
        let gsk = e.mem.alloc(0x140);
        e.mem.set_u32(gsk, first);
        e.mem.set_u32(gsk + 4, second);
        put(&mut e, gsk + 0x20, [4.0, 5.0, 6.0, 7.0]);
        put(&mut e, gsk + 0xa0, [1.0, 1.0, 1.0, 1.0]);
        put(&mut e, gsk + 0x130, [2.0, 2.0, 2.0, 2.0]);
        let transform_a = e.mem.alloc(0x40);
        put(&mut e, transform_a, [1.0, 0.0, 0.0, 0.0]);
        put(&mut e, transform_a + 0x10, [0.0, 2.0, 0.0, 0.0]);
        put(&mut e, transform_a + 0x20, [0.0, 0.0, 3.0, 0.0]);
        let transform_b = e.mem.alloc(0x40);
        let shape_a = e.mem.alloc(0x20);
        let shape_b = e.mem.alloc(0x20);
        e.mem.set_f32(shape_a + 0x10, 0.5);
        e.mem.set_f32(shape_b + 0x10, 0.25);
        let normal = e.mem.alloc(0x10);
        let second_out = e.mem.alloc(0x10);
        e.call(
            0x00db_52c0,
            &args![
                gsk,
                transform_a,
                transform_b,
                shape_a,
                shape_b,
                normal,
                second_out
            ],
        );
        let calls = calls_to(&e, SET_MUL_INVERSE_MUL);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][1..], [transform_a, transform_b]);
        let relative = calls[0][0];
        let closest = calls_to(&e, GET_CLOSEST_FEATURE);
        assert_eq!(closest.len(), 1);
        assert_eq!(closest[0][..4], [gsk, shape_a, shape_b, relative]);
        (load_lanes(&e, normal), load_lanes(&e, second_out))
    }

    /// The point `v` (three lanes) turned by the transposed rows `(1,2,3,4)`, `(5,6,7,8)`,
    /// `(9,10,11,12)`: columns `(1,5,9,10)`, `(2,6,10,12)`, `(3,7,11,12)`.
    fn turned(v: [f32; 3]) -> [u32; 4] {
        let columns = [
            [1.0f32, 5.0, 9.0, 10.0],
            [2.0, 6.0, 10.0, 12.0],
            [3.0, 7.0, 11.0, 12.0],
        ];
        [0, 1, 2, 3].map(|lane| {
            (v[1] * columns[1][lane] + v[0] * columns[0][lane] + columns[2][lane] * v[2]).to_bits()
        })
    }

    #[test]
    fn separating_normal_uses_the_first_simplex_vector_when_the_first_type_is_one() {
        let (normal, second) = separating(1, 0);
        // The direction (0, 1, 0) through the rows of the first transform: (0, 2, 0), its
        // fourth lane 2.5 - 0.5 - 0.25.
        assert_eq!(normal, [0.0f32, 2.0, 0.0, 1.75].map(f32::to_bits));
        // (4, 5, 6) - 2.5 * (0, 1, 0) - (0.5, 1, 1.5).
        assert_eq!(second, turned([3.5, 1.5, 4.5]));
    }

    #[test]
    fn separating_normal_adds_the_scaled_direction_for_the_second_type() {
        let (normal, second) = separating(0, 1);
        assert_eq!(normal, [0.0f32, 2.0, 0.0, 1.75].map(f32::to_bits));
        // (2.5 * A + (1, 1, 1)) - 2.5 * A - (0.5, 1, 1.5).
        assert_eq!(second, turned([0.5, 0.0, -0.5]));
    }

    #[test]
    fn separating_normal_falls_back_to_the_last_vector() {
        let (_, second) = separating(0, 0);
        assert_eq!(second, turned([1.5, -1.5, 0.5]));
    }

    /// A record whose first word points to an object with the vtable slots the state setup
    /// calls, and whose third word is a block with the float at `+0xa0`.
    fn record(e: &mut Engine, object: u32, block: u32) -> u32 {
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record, object);
        e.mem.set_u32(record + 8, block);
        record
    }

    #[test]
    fn state_setup_copies_the_blocks_and_builds_the_inertia_sums() {
        let mut e = rig();
        e.call_log = Some(Vec::new());
        e.mem.set_f64(EPSILON, 1.192_092_9e-7);
        let cursor_cell = e.mem.alloc(8);
        e.mem.set_u32(cursor_cell, 0x2000);
        e.register(MEMORY_ROUTER, |e, _| Ret {
            eax: e.mem.u32(0x0100_0000),
            ..Ret::default()
        });
        e.mem.set_u32(0x0100_0000, cursor_cell);
        e.register(0x0111_0000, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.register(0x0111_0004, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x2c, 0x0111_0000);
        e.mem.set_u32(vtable + 0x30, 0x0111_0004);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, vtable);
        e.mem.set_f32(object + 0x10, 0.5);
        let other = e.mem.alloc(0x20);
        e.mem.set_f32(other + 0x10, 0.25);
        let first_block = e.mem.alloc(0xb0);
        e.mem.set_f32(first_block + 0xa0, 5.0);
        let second_block = e.mem.alloc(0xb0);
        e.mem.set_f32(second_block + 0xa0, 3.0);
        let first = record(&mut e, object, first_block);
        let second = record(&mut e, other, second_block);
        let input = e.mem.alloc(0x80);
        e.mem.set_u32(input, first);
        e.mem.set_u32(input + 4, second);
        e.mem.set_u32(input + 0xc, 0x4000);
        put(&mut e, input + 0x70, [1.0, 2.0, 3.0, 4.0]);
        let state = e.mem.alloc(0x90);
        e.mem.set_f32(state + 0x6c, 1.0);
        e.mem.set_f32(state + 0x7c, 2.0);
        e.call(0x00db_53f0, &args![input, state]);

        assert_eq!(e.mem.u32(state), first_block);
        assert_eq!(e.mem.u32(state + 4), second_block);
        assert_eq!(e.mem.f32(state + 0x14), 0.75);
        assert_eq!(
            load_lanes(&e, state + 0x50),
            [1.0f32, 2.0, 3.0, 4.0].map(f32::to_bits)
        );
        assert_eq!(e.mem.u32(state + 0x40), 0x4050);
        assert_eq!(e.mem.u32(state + 0xc), 3);
        assert_eq!(e.mem.u32(state + 0x10), 3);
        // Three 16-byte elements round up to one 0x80 unit.
        assert_eq!(e.mem.u32(state + 8), 0x2000);
        assert_eq!(e.mem.u32(cursor_cell), 0x2080);
        assert_eq!(calls_to(&e, 0x0111_0004), vec![vec![object, 0x2000]]);
        // 3 * 2 * 2 + 5 * 1 * 1.
        assert_eq!(e.mem.f32(state + 0x18), 17.0);
        let reciprocal = e.mem.f32(state + 0x1c);
        assert!((reciprocal - 1.0 / 17.0).abs() < 1e-7, "{reciprocal}");
    }

    /// A collision scene for `collide_points_with_plane`: the interpolated transform is the
    /// identity (its fourth lane of the first row holds the fraction), the plane normal is
    /// `(1, 0, 0)` at a position `2 * fraction` along it, so a point's plane distance is its
    /// `x` minus twice the fraction. The first rotation row of the state is `(0, 0, 1)`, so a
    /// point's turning rate is minus its `y`. The plane limits are `1.0`, `0.9`, `0.0`.
    struct Scene {
        e: Engine,
        state: u32,
        plane: u32,
        out: u32,
        points: u32,
    }

    fn scene(points: &[[f32; 4]]) -> Scene {
        let mut e = rig();
        e.call_log = Some(Vec::new());
        e.register(INTERPOLATE_TRANSFORM, |e, a| {
            let fraction = f32::from_bits(a[2]);
            put(e, a[3], [1.0, 0.0, 0.0, fraction]);
            put(e, a[3] + 0x10, [0.0, 1.0, 0.0, 0.0]);
            put(e, a[3] + 0x20, [0.0, 0.0, 1.0, 0.0]);
            Ret::default()
        });
        e.register(SET_MUL_INVERSE_MUL, |_, _| Ret::default());
        e.register(SET_ROTATED_DIR, |e, a| {
            put(e, a[0], [1.0, 0.0, 0.0, 0.0]);
            Ret::default()
        });
        e.register(SET_TRANSFORMED_POS, |e, a| {
            let fraction = e.mem.f32(a[1] - 0x80 + 0xc);
            put(e, a[0], [2.0 * fraction, 0.0, 0.0, 0.0]);
            Ret::default()
        });
        e.register(TRANSFORM_UNNAMED_004B4AB0, |e, a| {
            put(e, a[0], [0.0; 4]);
            Ret::default()
        });
        let state = e.mem.alloc(0x90);
        let first_body = e.mem.alloc(0x90);
        let second_body = e.mem.alloc(0x90);
        let times = e.mem.alloc(0x10);
        e.mem.set_u32(state, first_body);
        e.mem.set_u32(state + 4, second_body);
        e.mem.set_u32(state + 0x40, times);
        put(&mut e, times, [0.0, 0.0, 1.0, 0.0]);
        let array = e.mem.alloc(16 * points.len() as u32);
        for (index, point) in points.iter().enumerate() {
            put(&mut e, array + 16 * index as u32, *point);
        }
        e.mem.set_u32(state + 8, array);
        e.mem.set_u32(state + 0xc, points.len() as u32);
        put(&mut e, state + 0x14, [0.0; 4]);
        e.mem.set_f32(state + 0x18, 100.0);
        e.mem.set_f32(state + 0x1c, 0.25);
        e.mem.set_f32(state + 0x44, 1.0);
        put(&mut e, state + 0x60, [0.0, 0.0, 1.0, 0.0]);
        put(&mut e, state + 0x70, [0.0; 4]);
        e.mem.set_f32(state + 0x80, 0.0);
        let plane = e.mem.alloc(0x20);
        put(&mut e, plane, [1.0, 0.9, 0.0, 0.0]);
        let out = e.mem.alloc(0x20);
        e.mem.set_f32(out, 10.0);
        put(&mut e, out + 0x10, [7.0; 4]);
        Scene {
            e,
            state,
            plane,
            out,
            points: array,
        }
    }

    impl Scene {
        fn run(&mut self) {
            let (state, plane, out) = (self.state, self.plane, self.out);
            self.e.call(0x00db_4e40, &args![state, plane, out]);
        }

        /// The fraction of the `index`th matrix initialisation.
        fn fraction(&self, index: usize) -> f32 {
            f32::from_bits(calls_to(&self.e, INTERPOLATE_TRANSFORM)[index][2])
        }
    }

    #[test]
    fn points_with_plane_without_points_does_nothing() {
        let mut s = scene(&[[0.5, 0.0, 0.0, 7.0]]);
        s.e.mem.set_u32(s.state + 0xc, 0);
        s.run();
        assert!(calls_to(&s.e, INTERPOLATE_TRANSFORM).is_empty());
        assert_eq!(s.e.mem.f32(s.out), 10.0);
    }

    #[test]
    fn points_with_plane_stores_a_point_in_front_of_the_plane_at_the_first_fraction() {
        // The distance 0.5 is below the limit 1.0 and the start fraction is the first one.
        let mut s = scene(&[[0.5, 0.0, 0.0, 7.0]]);
        s.run();
        assert_eq!(s.e.mem.f32(s.out), 0.0);
        assert_eq!(
            load_lanes(&s.e, s.out + 0x10),
            [0.5f32, 0.0, 0.0, 7.0].map(f32::to_bits)
        );
        // One round: the next fraction is the best time 1.0, but no point was kept.
        assert_eq!(calls_to(&s.e, INTERPOLATE_TRANSFORM).len(), 2);
    }

    #[test]
    fn points_with_plane_moves_a_far_point_to_the_front_with_its_distance() {
        let mut s = scene(&[[3.0, 0.0, 0.0, 7.0], [1.5, 0.0, 0.0, 8.0]]);
        s.run();
        // The point at distance 3.0 is beyond the limit plus the margin 1.0 and stays; the
        // second one (distance 1.5) is moved to the front, its fourth lane the distance.
        let array = load_lanes(&s.e, s.points);
        assert_eq!(array, [1.5f32, 0.0, 0.0, 1.5].map(f32::to_bits));
        let second = load_lanes(&s.e, s.points + 0x10);
        assert_eq!(second, [3.0f32, 0.0, 0.0, 7.0].map(f32::to_bits));
    }

    #[test]
    fn points_with_plane_picks_the_next_fraction_from_the_turning_rate() {
        // (y of the point, the expected second fraction). A point at distance 1.5 with the
        // turning rate -y: the time to the limit is sqrt(0.3) for a rate of 0 (plus
        // sqrt(0.05) for the gap), 2 * rate / 4 added for a positive rate, the larger of the
        // root and rate / 4 for a negative one; too slow a point gives the best time 1.0.
        let expected = [
            (0.0f32, 0.3f64.sqrt() + 0.05f64.sqrt()),
            (-0.1, 0.3f64.sqrt() + 0.05 + 0.05f64.sqrt()),
            (3.0, 0.75 + 0.05f64.sqrt()),
            (-2.0, 1.0),
        ];
        for (y, fraction) in expected {
            let mut s = scene(&[[1.5, y, 0.0, 7.0]]);
            s.run();
            let second = f64::from(s.fraction(2));
            assert!((second - fraction).abs() < 1e-4, "y {y}: {second}");
        }
    }

    #[test]
    fn points_with_plane_refines_the_time_of_impact_in_the_second_round() {
        // The distance 1.5 - 2 * fraction reaches the limit 1.0 at the fraction 0.25.
        let mut s = scene(&[[1.5, 0.0, 0.0, 7.0]]);
        s.run();
        let impact = s.e.mem.f32(s.out);
        assert!((impact - 0.25).abs() < 5e-3, "{impact}");
        assert_eq!(
            load_lanes(&s.e, s.out + 0x10),
            [1.5f32, 0.0, 0.0, 1.5].map(f32::to_bits)
        );
        assert!(calls_to(&s.e, INTERPOLATE_TRANSFORM).len() > 4);
    }
}
