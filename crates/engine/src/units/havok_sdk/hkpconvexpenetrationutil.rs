//! `Havok SDK/hkpconvexpenetrationutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkCalculatePenetrationDepth` (Xbox PDB) finds how deep two convex shapes overlap. It
//! builds a 0x70-byte search state (`00dfb1e0`), runs a simplex search on it (`00dfa940`) and,
//! when the search settles on the "needs the exact answer" outcome 3, asks `00e2db70` for it.
//! The state holds a 4x4 transform copy at +0x00 (four vectors), the radius splatted at
//! +0x40, its square at +0x50, an attempt counter at +0x5C and a first-attempt flag at +0x6C
//! (names by use; the Xbox PDB has no record of the state).
//!
//! The search (`00dfa940`) keeps its simplex on the stack: a header object at +0x20 of the
//! frame whose word at +8 counts the vertices, and up to 0x37 vertices of 0x40 bytes from
//! +0x40 on, in a frame of 0x2EC4 bytes. The callees (`00df9fe0`, `00dfaf80`, `00dfb0b0`,
//! `00df9b50`, `00df9c90`, `00df9d90`, `00df9ae0`) are not translated yet; they are called by
//! address.
//!
//! x87 note: `00dfa940` perturbs the state's vector at +0x30 after a failed attempt by
//! `(r - 0.0001) * 0.0002` (doubles, `r` the result of `00df9ae0` in ST0), rounded to
//! `float` and added to all four lanes with `ADDPS` against four frame words at +0x10, of
//! which the first three hold the perturbation and the fourth is whatever the frame held
//! (the exe never initialises it; the frame here starts zeroed, and the sum is stored back
//! into those four words, so later perturbations add the previous fourth lane again).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TlsGetValue` import slot and the TLS index of the Havok monitor stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// Profiling labels: `"TtPenetration"` and the end label.
const LABEL_PENETRATION: u32 = 0x010d_bb48;
const LABEL_END: u32 = 0x010c_41f8;
/// `double` 0.0001 (`01032980`) and 0.0002 (`010dbb40`) of the perturbation.
const PERTURBATION_OFFSET: u32 = 0x0103_2980;
const PERTURBATION_SCALE: u32 = 0x010d_bb40;
/// The search's callees. `this` is the state unless noted.
/// `(state, shapes, &outcome)` -> 1 when the search is decided.
const SEARCH_STEP: u32 = 0x00df_9fe0;
/// `this` = the simplex header: `(a, b, c)` taken from the state at +0x60, +0x64, +0x68.
const SIMPLEX_INIT: u32 = 0x00df_af80;
/// `this` = the simplex header: the next search direction, a pointer used as the vertex key.
const SIMPLEX_NEXT_DIRECTION: u32 = 0x00df_b0b0;
/// `(direction, vertex)`: fills a simplex vertex.
const FILL_VERTEX: u32 = 0x00df_9b50;
/// `(simplex header, direction, vertex, &outcome)` -> 1 when the simplex is decided.
const ADD_VERTEX: u32 = 0x00df_9c90;
/// `(simplex header, direction, shapes)`: the final computation after a decided search.
const FINISH: u32 = 0x00df_9d90;
/// Returns a `float` in ST0 (no arguments).
const RANDOM_VALUE: u32 = 0x00df_9ae0;
/// `(arg1, arg2, transform, arg5, arg6, arg9)`, cdecl: the exact answer for outcome 3.
const EXACT_DEPTH: u32 = 0x00e2_db70;

/// Vertices the simplex may hold before the search gives up with outcome 2.
const VERTEX_LIMIT: i32 = 0x37;
/// Attempts before the search gives up with outcome 3.
const ATTEMPT_LIMIT: i32 = 0x14;

layout! {
    /// The search state of `hkCalculatePenetrationDepth` (0x70 bytes), as built by `00dfb1e0`.
    pub struct PenetrationState: 0x70 {
        /// Fourth vector of the copied transform, perturbed by failed attempts.
        0x30 m_perturbed: Inline<[f32; 4]>,
        /// The radius splatted into four lanes.
        0x40 m_radius: Inline<[f32; 4]>,
        /// The radius squared.
        0x50 m_radiusSquared: f32,
        /// First of the two words set from the caller's arguments.
        0x54 m_field54: u32,
        /// Second of them.
        0x58 m_field58: u32,
        /// Attempt counter.
        0x5c m_attempts: i32,
        /// Words passed to the simplex start.
        0x60 m_field60: u32,
        0x64 m_field64: u32,
        0x68 m_field68: u32,
        /// 1 on the first attempt, 0 after a perturbation.
        0x6c m_firstAttempt: u32,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfa940, fn_00dfa940(Ptr<PenetrationState>, u32) -> i32),
        entry!(0x00dfaac0, hk_calculate_penetration_depth(u32, u32, Ptr, f32, u32, u32, u32, u32, u32) -> i32),
        entry!(
            0x00dfb1e0,
            fn_00dfb1e0(Ptr<PenetrationState>, u32, u32, Ptr, f32, u32, u32, u32, u32)
        ),
    ]
}

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte record `(label, time stamp)` to the thread's monitor stream while its
/// cursor (+4) is below its limit (+0xC).
fn profile_record(e: &mut Engine, label: u32) {
    let index: u32 = e.global(MONITOR_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

// Translated from 00dfb1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `RET 0x20`: builds the search state. Copies the 64 bytes of
/// `transform` to +0x00, splats `radius` into +0x40 (the argument's bits as pushed), stores
/// its square (computed on the x87 stack, so a signalling NaN is quieted) at +0x50,
/// `field_54` at +0x54, `field_58` at +0x58, 0 at +0x5C, and `field_60` to `field_6c` at
/// +0x60 to +0x6C.
#[allow(clippy::too_many_arguments)]
pub fn fn_00dfb1e0(
    e: &mut Engine,
    this: Ptr<PenetrationState>,
    field_54: u32,
    field_58: u32,
    transform: Ptr,
    radius: f32,
    field_60: u32,
    field_64: u32,
    field_68: u32,
    field_6c: u32,
) {
    let matrix = e.mem.bytes(transform.addr(), 0x40);
    e.mem.write(this.addr(), &matrix);
    let quiet = f64::from(x87_quiet(radius));
    e.set(
        this,
        PenetrationState::m_radiusSquared,
        (quiet * quiet) as f32,
    );
    e.set(this, PenetrationState::m_field54, field_54);
    e.set(this, PenetrationState::m_field58, field_58);
    for lane in 0..4 {
        e.mem
            .set_u32(this.addr() + 0x40 + 4 * lane, radius.to_bits());
    }
    e.set(this, PenetrationState::m_attempts, 0);
    e.set(this, PenetrationState::m_field60, field_60);
    e.set(this, PenetrationState::m_field64, field_64);
    e.set(this, PenetrationState::m_field68, field_68);
    e.set(this, PenetrationState::m_firstAttempt, field_6c);
}

/// The vertex key slot of vertex `index` in the search frame (vertices are 0x40 bytes from
/// +0x40; word +0x30 of each is cleared when it is taken).
fn vertex_at(frame: u32, index: i32) -> u32 {
    frame + 0x40 + 0x40 * index as u32
}

// Translated from 00dfa940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `RET 4`: the simplex search on a state, `(state, shapes)`.
/// Returns the outcome: whatever `00df9fe0` or the simplex reports when it is not 3, 2 when
/// the simplex needs more than 0x37 vertices (after the final computation `00df9d90`), and 3
/// when twenty attempts, each perturbing the state's vector at +0x30 (see the module
/// comment), still end in outcome 3.
pub fn fn_00dfa940(e: &mut Engine, this: Ptr<PenetrationState>, shapes: u32) -> i32 {
    e.with_stack(0x2ec4, |e, frame| {
        let frame = frame.addr();
        let outcome = frame + 0xc;
        let simplex = frame + 0x20;
        let vertex_count = frame + 0x28;
        loop {
            e.mem.set_i32(outcome, 0);
            let decided = e.call(SEARCH_STEP, &args![this, shapes, outcome]).u32();
            if decided == 1 {
                let result = e.mem.i32(outcome);
                if result != 3 {
                    return result;
                }
            } else {
                let first = e.get(this, PenetrationState::m_field60);
                let second = e.get(this, PenetrationState::m_field64);
                let third = e.get(this, PenetrationState::m_field68);
                e.call(SIMPLEX_INIT, &args![simplex, first, second, third]);
                let direction = loop {
                    let direction = e.call(SIMPLEX_NEXT_DIRECTION, &args![simplex]).u32();
                    let index = e.mem.i32(vertex_count);
                    let vertex = vertex_at(frame, index);
                    e.mem.set_i32(vertex_count, index.wrapping_add(1));
                    e.mem.set_u32(vertex + 0x30, 0);
                    e.call(FILL_VERTEX, &args![this, direction, vertex]);
                    let decided = e
                        .call(
                            ADD_VERTEX,
                            &args![this, simplex, direction, vertex, outcome],
                        )
                        .u32();
                    if decided == 1 {
                        break direction;
                    }
                    if e.mem.i32(vertex_count) >= VERTEX_LIMIT {
                        e.call(FINISH, &args![this, simplex, direction, shapes]);
                        return 2;
                    }
                };
                let result = e.mem.i32(outcome);
                if result != 3 {
                    e.call(FINISH, &args![this, simplex, direction, shapes]);
                    return result;
                }
            }
            // Outcome 3: try again, perturbed, up to the limit.
            let attempts = e.get(this, PenetrationState::m_attempts).wrapping_add(1);
            e.set(this, PenetrationState::m_attempts, attempts);
            if attempts == 1 {
                e.set(this, PenetrationState::m_firstAttempt, 1);
                continue;
            }
            if attempts >= ATTEMPT_LIMIT {
                return 3;
            }
            e.set(this, PenetrationState::m_firstAttempt, 0);
            let random = e.call(RANDOM_VALUE, &args![]).f64();
            let offset = e.global::<f64>(PERTURBATION_OFFSET);
            let scale = e.global::<f64>(PERTURBATION_SCALE);
            let step = ((random - offset) * scale) as f32;
            let scratch = frame + 0x10;
            for lane in 0..3 {
                e.mem.set_f32(scratch + 4 * lane, step);
            }
            for lane in 0..4 {
                let at = this.addr() + 0x30 + 4 * lane;
                let sum = e.mem.f32(at) + e.mem.f32(scratch + 4 * lane);
                e.mem.set_f32(scratch + 4 * lane, sum);
                e.mem.set_f32(at, sum);
            }
        }
    })
}

// Translated from 00dfaac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCalculatePenetrationDepth` (Xbox PDB), cdecl with nine words. Builds the search state
/// from `arg1`, `arg2`, `transform`, `radius` (a `float` pushed through `FLD`/`FSTP`) and
/// `arg5` to `arg8`, runs the search with `arg9`, and for outcome 3 asks `00e2db70` for the
/// exact depth and answers 1. Between `"TtPenetration"` profiling records.
#[allow(clippy::too_many_arguments)]
pub fn hk_calculate_penetration_depth(
    e: &mut Engine,
    arg1: u32,
    arg2: u32,
    transform: Ptr,
    radius: f32,
    arg5: u32,
    arg6: u32,
    arg7: u32,
    arg8: u32,
    arg9: u32,
) -> i32 {
    profile_record(e, LABEL_PENETRATION);
    let outcome = e.with_stack(0x70, |e, state| {
        let state = state.cast::<PenetrationState>();
        fn_00dfb1e0(
            e,
            state,
            arg1,
            arg2,
            transform,
            x87_quiet(radius),
            arg5,
            arg6,
            arg7,
            arg8,
        );
        let mut outcome = fn_00dfa940(e, state, arg9);
        if outcome == 3 {
            e.call(EXACT_DEPTH, &args![arg1, arg2, transform, arg5, arg6, arg9]);
            outcome = 1;
        }
        outcome
    });
    profile_record(e, LABEL_END);
    outcome
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// The monitor stream the profiling records go to.
    const STREAM: u32 = 0x0300_0000;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f64(PERTURBATION_OFFSET, 0.0001);
        e.mem.set_f64(PERTURBATION_SCALE, 0.0002);
        e.map(STREAM, 0x200);
        let stream = STREAM;
        e.mem.set_u32(stream + 4, stream + 0x40);
        e.mem.set_u32(stream + 0xc, stream + 0x1f0);
        e.mem.set_u32(MONITOR_SLOT, 5);
        e.register_double(TLS_GET_VALUE, move |_, _| stream.into_ret());
        for addr in [SIMPLEX_INIT, FILL_VERTEX, FINISH, EXACT_DEPTH, RANDOM_VALUE] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A search step that answers "decided" with the outcomes of `script`, in turn.
    fn scripted_step(e: &mut Engine, script: Vec<(u32, i32)>) {
        let turn = Rc::new(Cell::new(0usize));
        e.register_double(SEARCH_STEP, move |e, a| {
            let (decided, outcome) = script[turn.get().min(script.len() - 1)];
            turn.set(turn.get() + 1);
            e.mem.set_i32(a[2], outcome);
            decided.into_ret()
        });
    }

    fn state(e: &mut Engine) -> Ptr<PenetrationState> {
        e.new_object::<PenetrationState>()
    }

    #[test]
    fn state_is_built_from_the_arguments() {
        let mut e = rig();
        let this = state(&mut e);
        let transform = e.mem.alloc(0x40);
        for i in 0..16 {
            e.mem.set_f32(transform + 4 * i, i as f32 + 1.0);
        }
        e.call(
            0x00df_b1e0,
            &args![
                this,
                0xa1u32,
                0xa2u32,
                Ptr::<()>::new(transform),
                3.0f32,
                5u32,
                6u32,
                7u32,
                8u32
            ],
        );
        for i in 0..16 {
            assert_eq!(e.mem.f32(this.addr() + 4 * i), i as f32 + 1.0);
        }
        for lane in 0..4 {
            assert_eq!(e.mem.f32(this.addr() + 0x40 + 4 * lane), 3.0);
        }
        assert_eq!(e.get(this, PenetrationState::m_radiusSquared), 9.0);
        assert_eq!(e.get(this, PenetrationState::m_field54), 0xa1);
        assert_eq!(e.get(this, PenetrationState::m_field58), 0xa2);
        assert_eq!(e.get(this, PenetrationState::m_attempts), 0);
        assert_eq!(e.get(this, PenetrationState::m_field60), 5);
        assert_eq!(e.get(this, PenetrationState::m_field64), 6);
        assert_eq!(e.get(this, PenetrationState::m_field68), 7);
        assert_eq!(e.get(this, PenetrationState::m_firstAttempt), 8);
    }

    #[test]
    fn a_decided_step_other_than_three_is_the_answer() {
        let mut e = rig();
        let this = state(&mut e);
        scripted_step(&mut e, vec![(1, 2)]);
        assert_eq!(e.call(0x00df_a940, &args![this, 0x77u32]).i32(), 2);
        assert!(calls(&e, SIMPLEX_INIT).is_empty());
        assert_eq!(calls(&e, SEARCH_STEP)[0][..2], [this.addr(), 0x77]);
    }

    #[test]
    fn the_simplex_runs_when_the_step_is_undecided() {
        let mut e = rig();
        let this = state(&mut e);
        e.set(this, PenetrationState::m_field60, 11);
        e.set(this, PenetrationState::m_field64, 12);
        e.set(this, PenetrationState::m_field68, 13);
        scripted_step(&mut e, vec![(0, 0)]);
        e.register(SIMPLEX_NEXT_DIRECTION, |_, _| 0xd00du32.into_ret());
        // The simplex is decided on the third vertex, with outcome 1.
        let added = Rc::new(Cell::new(0));
        let counter = Rc::clone(&added);
        let vertices = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&vertices);
        e.register_double(ADD_VERTEX, move |e, a| {
            counter.set(counter.get() + 1);
            seen.borrow_mut().push((a[1], a[3]));
            if counter.get() == 3 {
                e.mem.set_i32(a[4], 1);
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        assert_eq!(e.call(0x00df_a940, &args![this, 0x77u32]).i32(), 1);
        let init = &calls(&e, SIMPLEX_INIT)[0];
        assert_eq!(init[1..], [11, 12, 13]);
        let vertices = vertices.borrow();
        assert_eq!(vertices.len(), 3);
        // Vertices are 0x40 bytes apart in the frame, after the header.
        assert_eq!(vertices[1].1 - vertices[0].1, 0x40);
        assert_eq!(vertices[0].1 - init[0], 0x20);
        // The final computation gets the simplex, the direction and the shapes.
        assert_eq!(
            calls(&e, FINISH),
            vec![vec![this.addr(), init[0], 0xd00d, 0x77]]
        );
    }

    #[test]
    fn too_many_vertices_answer_two_after_the_final_computation() {
        let mut e = rig();
        let this = state(&mut e);
        scripted_step(&mut e, vec![(0, 0)]);
        e.register(SIMPLEX_NEXT_DIRECTION, |_, _| 0xd00du32.into_ret());
        e.register(ADD_VERTEX, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00df_a940, &args![this, 1u32]).i32(), 2);
        assert_eq!(calls(&e, ADD_VERTEX).len(), VERTEX_LIMIT as usize);
        assert_eq!(calls(&e, FINISH).len(), 1);
    }

    #[test]
    fn outcome_three_is_retried_with_perturbations_and_then_given_up() {
        let mut e = rig();
        let this = state(&mut e);
        e.mem.set_f32(this.addr() + 0x30, 1.0);
        e.mem.set_f32(this.addr() + 0x3c, 1.0);
        scripted_step(&mut e, vec![(1, 3)]);
        // r = 0.5001 gives a perturbation of (0.5001 - 0.0001) * 0.0002 = 0.0001.
        e.register(RANDOM_VALUE, |_, _| Ret {
            st0: 0.5001,
            ..Ret::default()
        });
        assert_eq!(e.call(0x00df_a940, &args![this, 1u32]).i32(), 3);
        assert_eq!(calls(&e, SEARCH_STEP).len(), ATTEMPT_LIMIT as usize);
        assert_eq!(e.get(this, PenetrationState::m_attempts), ATTEMPT_LIMIT);
        assert_eq!(e.get(this, PenetrationState::m_firstAttempt), 0);
        // Eighteen perturbations of 0.0001 (the first attempt only sets the flag).
        let moved = e.mem.f32(this.addr() + 0x30);
        assert!((moved - (1.0 + 18.0 * 0.0001)).abs() < 1e-4, "{moved}");
        // The fourth lane adds the frame's previous fourth lane each time: it doubles.
        assert_eq!(e.mem.f32(this.addr() + 0x3c), 1.0 * 2f32.powi(17));
    }

    #[test]
    fn penetration_depth_asks_for_the_exact_answer_on_three() {
        let mut e = rig();
        let transform = e.mem.alloc(0x40);
        scripted_step(&mut e, vec![(1, 3), (1, 3), (1, 3)]);
        // Make the search give up at once: outcome 3 for the whole run.
        let result = e
            .call(
                0x00df_aac0,
                &args![
                    1u32,
                    2u32,
                    Ptr::<()>::new(transform),
                    0.5f32,
                    5u32,
                    6u32,
                    7u32,
                    8u32,
                    9u32
                ],
            )
            .i32();
        assert_eq!(result, 1);
        assert_eq!(calls(&e, EXACT_DEPTH), vec![vec![1, 2, transform, 5, 6, 9]]);
        // The search ran with the ninth argument as its shapes.
        assert_eq!(calls(&e, SEARCH_STEP)[0][1], 9);
    }

    #[test]
    fn penetration_depth_passes_other_outcomes_through() {
        let mut e = rig();
        let transform = e.mem.alloc(0x40);
        scripted_step(&mut e, vec![(1, 2)]);
        let result = e
            .call(
                0x00df_aac0,
                &args![
                    1u32,
                    2u32,
                    Ptr::<()>::new(transform),
                    0.5f32,
                    5u32,
                    6u32,
                    7u32,
                    8u32,
                    9u32
                ],
            )
            .i32();
        assert_eq!(result, 2);
        assert!(calls(&e, EXACT_DEPTH).is_empty());
        // The start and end profiling records.
        assert_eq!(e.mem.u32(STREAM + 4), STREAM + 0x40 + 24);
        assert_eq!(e.mem.u32(STREAM + 0x40), LABEL_PENETRATION);
        assert_eq!(e.mem.u32(STREAM + 0x4c), LABEL_END);
    }
}
