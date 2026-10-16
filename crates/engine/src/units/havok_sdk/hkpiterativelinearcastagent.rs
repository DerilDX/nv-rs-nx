//! `Havok SDK/hkpiterativelinearcastagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The large float `0x7f7fffee` (`0104da48`) used as "no hit yet".
const NO_HIT: u32 = 0x0104_da48;
/// The double `0.0` (`01012060`) the final test compares against.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The vtable pointer stored first in the local hit records (`010cebe0`).
const RECORD_VTABLE: u32 = 0x010c_ebe0;
/// `hkMotionState::hkMotionState` (Xbox PDB, `00d431b0`, thiscall, one pointer argument).
const MOTION_STATE_COPY: u32 = 0x00d4_31b0;

type Vec4 = [f32; 4];

fn ld(e: &Engine, at: u32) -> Vec4 {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn st(e: &mut Engine, at: u32, v: Vec4) {
    for (i, lane) in v.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, lane);
    }
}

fn copy16(e: &mut Engine, from: u32, to: u32) {
    for i in 0..4 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// An `FLD` of a float in memory (it quiets a signalling NaN).
fn load_float(e: &Engine, at: u32) -> f32 {
    x87_quiet(e.mem.f32(at))
}

/// The dot product of the first three lanes the way the exe sums them: lane 1 plus lane 0,
/// then lane 2 plus that.
fn dot3(a: Vec4, b: Vec4) -> f32 {
    let p = [a[0] * b[0], a[1] * b[1], a[2] * b[2]];
    p[2] + (p[1] + p[0])
}

// Translated from 00daf540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpIterativeLinearCastAgent::linearCast` (Xbox PDB), thiscall `(this, body, other, input,
/// output, early_out)`; the callee pops five words after `this`. Unconfirmed names: `body`,
/// `other`, `input`, `output` and `early_out` are the identities the call pattern suggests.
/// The input is copied with its third float (`input[2]`) raised by `input[0x19]` and the
/// virtual function at slot `0x10` of `this` is asked for the closest point (arguments `body`,
/// `other`, the copy, a local hit record whose byte at +8 reports a hit and whose float at
/// +0x2c holds the distance `d`). With a hit: `early_out` (slot 4 of its vtable, when non-null)
/// is told if `input[2] > d`. With `s = dot(input + 0x50, record + 0x20)`, the cast returns
/// when `d + s > 0` or `s + input[0x18]` is not below zero. Otherwise, when the first float of
/// the list at `input[0x1a]` is not below `d`, the hit fraction `d / (d - (d + s))` (zero when
/// `d <= 0`) goes to `output` through slot 4 of its vtable unless it exceeds `output[1]`.
/// When it is below `d`, the cast iterates: a local `hkMotionState` is built from `body[2]`,
/// and for each list entry (count at `list[1]`, counting down) the state is moved along the
/// cast by the current fraction, the closest point is asked again, and the fraction is
/// refined by `d2 / -dot(input + 0x50, record2 + 0x20)`; the loop ends when the refinement
/// exceeds the allowed distance or `output[1]`, or when the list's first float is not below
/// `d2`; the last hit goes to `output` (slot 4).
///
/// Uninitialised stack words: the second hit record (`second` here, at `f + 0x84`) is only
/// initialised in its vtable word, hit byte and the two floats at +4 and +0x2c; the loop
/// reads its normal (`second + 0x20`, `MOVAPS` at `00daf7d1`) without testing the hit byte, so
/// when the closest-point query misses on the first pass the exe uses stack garbage there (its
/// position at +0x10 can reach `hit` through the copies at `00daf872` the same way). Zeros
/// stand in, which ends the loop (`slope < 0` fails); a garbage normal can take other paths.
// Translation incomplete: 00daf540 - reads uninitialised stack word at ESP+0xb0 (second hit record normal, 00daf7d1)
pub fn hkp_iterative_linear_cast_agent_linear_cast(
    e: &mut Engine,
    this: Ptr,
    body: Ptr,
    other: Ptr,
    input: Ptr,
    output: Ptr,
    early_out: Ptr,
) {
    let t = this.addr();
    let (a1, a2) = (body.addr(), other.addr());
    let inp = input.addr();
    let (out4, out5) = (output.addr(), early_out.addr());
    e.with_stack(0x200, |e, frame| {
        let f = frame.addr();
        let record = f + 0x54;
        let hit = f + 0x14;
        let copy = f + 0xc4;
        let motion = f + 0x114;
        let big = load_float(e, NO_HIT);
        let raise = load_float(e, inp + 0x64);
        for i in 0..20 {
            let word = e.mem.u32(inp + 4 * i);
            e.mem.set_u32(copy + 4 * i, word);
        }
        let raised = (f64::from(raise) + f64::from(load_float(e, copy + 8))) as f32;
        e.mem.set_f32(copy + 8, raised);
        e.mem.set_f32(record + 0x2c, big);
        e.mem.set_f32(record + 4, big);
        e.mem.set_u32(record, RECORD_VTABLE);
        e.mem.set_u8(record + 8, 0);
        e.vcall(t, 0x10, &args![a1, a2, copy, record]);
        if e.mem.u8(record + 8) == 0 {
            return;
        }
        let mut d = load_float(e, record + 0x2c);
        copy16(e, record + 0x10, hit);
        copy16(e, record + 0x20, hit + 0x10);
        e.mem.set_u32(hit + 0x20, a1);
        e.mem.set_u32(hit + 0x24, a2);
        let limit = load_float(e, inp + 8);
        if limit > d && out5 != 0 {
            e.vcall(out5, 4, &args![hit]);
            d = load_float(e, record + 0x2c);
        }
        let direction = ld(e, inp + 0x50);
        st(e, f + 0xb4, direction);
        let s = x87_quiet(dot3(direction, ld(e, record + 0x20)));
        let f1 = (f64::from(d) + f64::from(s)) as f32;
        if f1 > 0.0 {
            return;
        }
        let u = load_float(e, inp + 0x60);
        let reach = f64::from(s) + f64::from(u);
        let below_zero = reach < e.mem.f64(ZERO_DOUBLE);
        if !below_zero {
            return;
        }
        let list = e.mem.u32(inp + 0x68);
        if load_float(e, list) >= d {
            if 0.0 < d {
                let fraction = (f64::from(d) / (f64::from(d) - f64::from(f1))) as f32;
                if load_float(e, out4 + 4) < fraction {
                    return;
                }
                e.mem.set_f32(hit + 0x1c, fraction);
            } else {
                e.mem.set_f32(hit + 0x1c, 0.0);
            }
            e.vcall(out4, 4, &args![hit]);
            return;
        }
        let fraction = (f64::from(d) / (f64::from(d) - f64::from(f1))) as f32;
        e.mem.set_f32(hit + 0x1c, fraction);
        let second = f + 0x84;
        e.mem.set_u32(second, RECORD_VTABLE);
        e.mem.set_u8(second + 8, 0);
        e.mem.set_f32(second + 0x2c, big);
        e.mem.set_f32(second + 4, big);
        let source_motion = e.mem.u32(a1 + 8);
        e.call(MOTION_STATE_COPY, &args![motion, source_motion]);
        let body_record = f + 0x44;
        let first_word = e.mem.u32(a1);
        e.mem.set_u32(body_record + 0xc, a1);
        e.mem.set_u32(body_record + 8, motion);
        e.mem.set_u32(body_record, first_word);
        e.mem.set_u32(body_record + 4, 0xffff_ffff);
        let mut remaining = (e.mem.u32(list + 4) as i32).wrapping_sub(1);
        if remaining >= 0 {
            let mut step = ld(e, f + 0xb4);
            loop {
                e.mem.set_f32(second + 0x2c, big);
                e.mem.set_f32(second + 4, big);
                let current = load_float(e, hit + 0x1c);
                e.mem.set_u8(second + 8, 0);
                let state = e.mem.u32(a1 + 8);
                let position = ld(e, state + 0x30);
                let moved = [0, 1, 2, 3].map(|i| current * step[i] + position[i]);
                st(e, motion + 0x30, moved);
                e.vcall(t, 0x10, &args![body_record, a2, copy, second]);
                step = ld(e, inp + 0x50);
                let slope = x87_quiet(dot3(step, ld(e, second + 0x20)));
                let negative = slope < 0.0;
                if !negative {
                    return;
                }
                let drop = -slope;
                let product = (f64::from(current) * f64::from(drop)) as f32;
                let d2 = load_float(e, second + 0x2c);
                if f64::from(product) + f64::from(d2) > f64::from(drop) {
                    return;
                }
                let refined = (f64::from(current) + f64::from(d2) / f64::from(drop)) as f32;
                if load_float(e, out4 + 4) < refined {
                    return;
                }
                copy16(e, second + 0x20, hit + 0x10);
                e.mem.set_f32(hit + 0x1c, refined);
                copy16(e, second + 0x10, hit);
                let list_now = e.mem.u32(inp + 0x68);
                let first = load_float(e, list_now);
                if first < d2 {
                    remaining -= 1;
                    if remaining >= 0 {
                        continue;
                    }
                }
                break;
            }
        }
        e.vcall(out4, 4, &args![hit]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00da_f540,
        hkp_iterative_linear_cast_agent_linear_cast(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
    )]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    const FIRST_FINDER: u32 = 0x00ee_1000;
    const COLLECTOR: u32 = 0x00ee_1001;
    const EARLY_OUT: u32 = 0x00ee_1002;

    /// What a collector callback saw: the hit record's vectors and the two words.
    type Seen = Rc<RefCell<Vec<(Vec4, Vec4, u32, u32)>>>;
    type Finder = Box<dyn Fn(&mut Engine, &[u32], u32)>;

    struct Rig {
        e: Engine,
        this: u32,
        body: u32,
        other: u32,
        input: u32,
        output: u32,
        early: u32,
        collected: Seen,
        early_calls: Rc<Cell<u32>>,
    }

    fn object_with_slot(e: &mut Engine, offset: u32, address: u32) -> u32 {
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + offset, address);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, table);
        object
    }

    /// `finder` runs for each closest-point request, with the request's words and its number.
    fn rig(list_first: f32, list_count: u32, finder: Finder) -> Rig {
        let mut e = test_engine();
        e.mem.set_f32(NO_HIT, f32::from_bits(0x7f7f_ffee));
        e.mem.set_f64(ZERO_DOUBLE, 0.0);
        let this = object_with_slot(&mut e, 0x10, FIRST_FINDER);
        let calls = Rc::new(Cell::new(0u32));
        let counter = Rc::clone(&calls);
        e.register_double(FIRST_FINDER, move |e, a| {
            let n = counter.get();
            counter.set(n + 1);
            finder(e, a, n);
            Ret::default()
        });
        let output = object_with_slot(&mut e, 4, COLLECTOR);
        e.mem.set_f32(output + 4, 1.0);
        let collected: Seen = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&collected);
        e.register_double(COLLECTOR, move |e, a| {
            let h = a[1];
            let record = (
                ld(e, h),
                ld(e, h + 0x10),
                e.mem.u32(h + 0x20),
                e.mem.u32(h + 0x24),
            );
            seen.borrow_mut().push(record);
            Ret::default()
        });
        let early = object_with_slot(&mut e, 4, EARLY_OUT);
        let early_calls = Rc::new(Cell::new(0u32));
        let early_counter = Rc::clone(&early_calls);
        e.register_double(EARLY_OUT, move |_, _| {
            early_counter.set(early_counter.get() + 1);
            Ret::default()
        });
        let motion = e.mem.alloc(0x40);
        st(&mut e, motion + 0x30, [1.0, 2.0, 3.0, 0.0]);
        let shape = e.mem.alloc(0x10);
        let body = e.mem.alloc(0x20);
        e.mem.set_u32(body, shape);
        e.mem.set_u32(body + 8, motion);
        let other = e.mem.alloc(0x20);
        let list = e.mem.alloc(0x10);
        e.mem.set_f32(list, list_first);
        e.mem.set_u32(list + 4, list_count);
        let input = e.mem.alloc(0x80);
        e.mem.set_f32(input + 8, 2.0);
        st(&mut e, input + 0x50, [0.0, 0.0, -1.0, 0.0]);
        e.mem.set_f32(input + 0x60, 0.25);
        e.mem.set_f32(input + 0x64, 0.0);
        e.mem.set_u32(input + 0x68, list);
        Rig {
            e,
            this,
            body,
            other,
            input,
            output,
            early,
            collected,
            early_calls,
        }
    }

    fn run(r: &mut Rig, early: u32) {
        r.e.call(
            0x00da_f540,
            &args![r.this, r.body, r.other, r.input, r.output, early],
        );
    }

    /// A finder that reports a hit with distance `d` in the first request and `d2` after.
    fn hit_at(d: f32, d2: f32) -> Finder {
        Box::new(move |e, a, n| {
            let record = a[4];
            if n == 0 {
                e.mem.set_u8(record + 8, 1);
                st(e, record + 0x20, [0.0, 0.0, 1.0, d]);
                st(e, record + 0x10, [7.0, 7.0, 7.0, 7.0]);
            } else {
                st(e, record + 0x20, [0.0, 0.0, 1.0, d2]);
                st(e, record + 0x10, [8.0, 8.0, 8.0, 8.0]);
            }
        })
    }

    #[test]
    fn no_hit_leaves_without_a_callback() {
        let mut r = rig(2.0, 1, Box::new(|_, _, _| {}));
        run(&mut r, 0);
        assert!(r.collected.borrow().is_empty());
        assert_eq!(r.early_calls.get(), 0);
    }

    #[test]
    fn a_direct_hit_reports_the_fraction_and_tells_the_early_out() {
        let mut r = rig(2.0, 1, hit_at(0.5, 0.0));
        let early = r.early;
        run(&mut r, early);
        // d = 0.5, s = -1: the fraction is 0.5 / (0.5 - (-0.5)).
        let seen = r.collected.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0, [7.0; 4]);
        assert_eq!(seen[0].1, [0.0, 0.0, 1.0, 0.5]);
        assert_eq!(seen[0].2, r.body);
        assert_eq!(seen[0].3, r.other);
        // input[2] (2.0) exceeds d, so the early-out listener is told once.
        assert_eq!(r.early_calls.get(), 1);
    }

    #[test]
    fn a_hit_at_or_behind_the_start_reports_fraction_zero_and_a_far_one_is_dropped() {
        let mut r = rig(2.0, 1, hit_at(-0.2, 0.0));
        run(&mut r, 0);
        {
            let seen = r.collected.borrow();
            assert_eq!(seen.len(), 1);
            assert_eq!(seen[0].1[3], 0.0);
        }
        assert_eq!(r.early_calls.get(), 0);
        // The fraction 0.5 exceeds the collector's limit, so nothing is reported.
        let mut r = rig(2.0, 1, hit_at(0.5, 0.0));
        let output = r.output;
        r.e.mem.set_f32(output + 4, 0.25);
        run(&mut r, 0);
        assert!(r.collected.borrow().is_empty());
    }

    #[test]
    fn a_hit_moving_away_is_ignored() {
        // d + s > 0: d = 3.0 with s = -1 is still positive.
        let mut r = rig(5.0, 1, hit_at(3.0, 0.0));
        run(&mut r, 0);
        assert!(r.collected.borrow().is_empty());
    }

    #[test]
    fn the_iteration_moves_the_state_and_reports_the_refined_fraction() {
        type Moved = Rc<RefCell<Option<(Vec4, u32, bool)>>>;
        let moved: Moved = Rc::new(RefCell::new(None));
        let sink = Rc::clone(&moved);
        let finder = hit_at(0.5, 0.1);
        let mut r = rig(
            0.25,
            1,
            Box::new(move |e, a, n| {
                if n == 1 {
                    // The state copy sits right after the 80-byte input copy.
                    let state = a[3] + 0x50;
                    let body_record = a[1];
                    let seen = (
                        ld(e, state + 0x30),
                        e.mem.u32(body_record + 4),
                        e.mem.u32(body_record + 8) == state,
                    );
                    *sink.borrow_mut() = Some(seen);
                }
                finder(e, a, n);
            }),
        );
        let motion_copies = Rc::new(Cell::new(0u32));
        let counter = Rc::clone(&motion_copies);
        r.e.register_double(MOTION_STATE_COPY, move |_, _| {
            counter.set(counter.get() + 1);
            Ret::default()
        });
        run(&mut r, 0);
        assert_eq!(motion_copies.get(), 1);
        // 0.5 * (0, 0, -1) + (1, 2, 3).
        let (position, key, linked) = moved.borrow().unwrap();
        assert_eq!(position, [1.0, 2.0, 2.5, 0.0]);
        assert_eq!(key, 0xffff_ffff);
        assert!(linked);
        // d2 = 0.1, slope -1: the refined fraction is 0.5 + 0.1.
        let seen = r.collected.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0, [8.0; 4]);
        assert!((seen[0].1[3] - 0.6).abs() < 1e-6);
    }
}
