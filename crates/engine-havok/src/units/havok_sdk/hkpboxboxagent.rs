//! `Havok SDK/hkpboxboxagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpBoxBoxAgent` (the Xbox PDB has no record of the Havok classes, so the field names below
//! are unconfirmed). Its three work functions each build the same query record for the
//! box-box collision detection (`hkpBoxBoxCollisionDetection`, 0xb8 bytes) on the stack:
//!
//! | offset | content |
//! |--------|---------|
//! | `+0x00`, `+0x04` | the two bodies |
//! | `+0x08`, `+0x0C`, `+0x10` | per function (0, or the input, a word of the agent and the result) |
//! | `+0x14`, `+0x18` | the two bodies' transforms (`body + 8`) |
//! | `+0x20` | four vectors: the transform B-in-A (`hkTransform::setMulInverseMul`) |
//! | `+0x60`, `+0x70` | `radius + halfExtents` of each box |
//! | `+0x80` | the tolerance in four lanes |
//! | `+0x90`, `+0xA0` | `tolerance + (radius + halfExtents)` of each box |
//! | `+0xB0`, `+0xB4` | the tolerance, and the float constant at `01013ea4` |
//!
//! A body is `{ shape, ..., transform at +8 }`; a box shape has its radius at `+0x10` and its
//! half extents (a vector) at `+0x20`. The radius and tolerance reach the record through the
//! x87 stack (`FLD`/`FSTP`, which quiets a signalling NaN); the sums are SSE lane by lane.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpsphereshape::{copy_vec, load_vec, profile_record, store_vec, Vec4, PROFILE_END};

/// The profiling label `"TtBoxBox"` (`010cf208`) the three work functions begin with.
const PROFILE_BEGIN: u32 = 0x010c_f208;
/// `hkTransform::setMulInverseMul` (`00cb22f0`): `this` is the output, two words (the transforms).
const SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
/// The float constant (0.01) stored at `+0xB4` of the query record.
const QUERY_CONSTANT: u32 = 0x0101_3ea4;
/// `hkpBoxBoxCollisionDetection::calculateClosestPoint` (`00db2750`, `this` the query record,
/// two words: a result flag byte, a result point), `checkIntersection` (`00db0ec0`, one word)
/// and `calcManifold` (`00db2960`, one word); `00d40940` prepares the record (no arguments).
const CALCULATE_CLOSEST_POINT: u32 = 0x00db_2750;
const CHECK_INTERSECTION: u32 = 0x00db_0ec0;
const CALC_MANIFOLD: u32 = 0x00db_2960;
const PREPARE_QUERY: u32 = 0x00d4_0940;

/// Where the pieces of one query sit (absolute addresses in the function's frame).
struct QueryFrame {
    /// The output of `setMulInverseMul` (four vectors).
    transform: u32,
    /// The two `radius + halfExtents` scratch vectors.
    extents_a: u32,
    extents_b: u32,
    /// The query record.
    query: u32,
}

/// The part every work function shares: the extents, the transform of B in A and the
/// tolerance go into the query record (`+0x00`, `+0x04`, `+0x14` to `+0xB4`).
fn build_query(e: &mut Engine, frame: &QueryFrame, body_a: u32, body_b: u32, input: u32) {
    let shape_a = e.mem.u32(body_a);
    let radius_a = x87_quiet(e.mem.f32(shape_a + 0x10));
    let half_a = load_vec(e, shape_a + 0x20);
    let shape_b = e.mem.u32(body_b);
    let radius_b = x87_quiet(e.mem.f32(shape_b + 0x10));
    let half_b = load_vec(e, shape_b + 0x20);
    let transform_b = e.mem.u32(body_b + 8);
    let extents_a: Vec4 = std::array::from_fn(|lane| radius_a + half_a[lane]);
    store_vec(e, frame.extents_a, extents_a);
    let transform_a = e.mem.u32(body_a + 8);
    let extents_b: Vec4 = std::array::from_fn(|lane| radius_b + half_b[lane]);
    store_vec(e, frame.extents_b, extents_b);
    e.call(
        SET_MUL_INVERSE_MUL,
        &args![frame.transform, transform_a, transform_b],
    );

    let query = frame.query;
    let tolerance_bits = x87_quiet_bits(e.mem.u32(input + 8));
    let tolerance = f32::from_bits(tolerance_bits);
    for row in 0..4 {
        copy_vec(e, frame.transform + 0x10 * row, query + 0x20 + 0x10 * row);
    }
    e.mem.set_u32(query + 0xb0, tolerance_bits);
    let constant_bits = x87_quiet_bits(e.mem.u32(QUERY_CONSTANT));
    e.mem.set_u32(query + 0xb4, constant_bits);
    e.mem.set_u32(query, body_a);
    e.mem.set_u32(query + 4, body_b);
    e.mem.set_u32(query + 0x14, transform_a);
    e.mem.set_u32(query + 0x18, transform_b);
    store_vec(e, query + 0x60, extents_a);
    store_vec(e, query + 0x70, extents_b);
    store_vec(e, query + 0x80, [tolerance; 4]);
    let with_tolerance_a: Vec4 = std::array::from_fn(|lane| tolerance + extents_a[lane]);
    store_vec(e, query + 0x90, with_tolerance_a);
    let with_tolerance_b: Vec4 = std::array::from_fn(|lane| tolerance + extents_b[lane]);
    store_vec(e, query + 0xa0, with_tolerance_b);
}

// Translated from 00d40180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxBoxAgent::cleanup` (engine map): for each of the `count` (byte at `+0x31`) entries
/// of the id table (words at `+0x12`, 4 bytes apart) calls the owner's (`+8`) slot `0x14` with
/// the id and `argument`, then deletes the agent through its own slot 0 (flag 1).
pub fn hkp_box_box_agent_cleanup(e: &mut Engine, this: Ptr, argument: u32) {
    let agent = this.addr();
    if e.mem.u8(agent + 0x31) != 0 {
        let mut index = 0u32;
        let mut slot = agent + 0x12;
        loop {
            let owner = e.mem.u32(agent + 8);
            let id = u32::from(e.mem.u16(slot));
            e.vcall(owner, 0x14, &args![id, argument]);
            index += 1;
            slot += 4;
            if index >= u32::from(e.mem.u8(agent + 0x31)) {
                break;
            }
        }
    }
    e.vcall(agent, 0, &args![1u32]);
}

// Translated from 00d40280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxBoxAgent::processCollision` (engine map): between two profiling records
/// (`"TtBoxBox"` and the end label) builds the query record for `body_a` and `body_b` with the
/// input's tolerance (`input + 8`), `input` at `+8`, the word at `this + 8` at `+0x0C` and
/// `result` at `+0x10`, and runs `calcManifold` on it with `this + 0x10`.
pub fn hkp_box_box_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    input: u32,
    result: u32,
) {
    let agent = this.addr();
    e.with_stack(0x194, |e, frame| {
        let f = frame.addr();
        profile_record(e, PROFILE_BEGIN);
        let query_frame = QueryFrame {
            transform: f + 0x24,
            extents_a: f + 0x14,
            extents_b: f + 4,
            query: f + 0x64,
        };
        build_query(e, &query_frame, body_a, body_b, input);
        let agent_word = e.mem.u32(agent + 8);
        e.mem.set_u32(query_frame.query + 8, input);
        e.mem.set_u32(query_frame.query + 0xc, agent_word);
        e.mem.set_u32(query_frame.query + 0x10, result);
        e.call(CALC_MANIFOLD, &args![query_frame.query, agent + 0x10]);
        profile_record(e, PROFILE_END);
    });
}

// Translated from 00d40470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxBoxAgent::staticGetClosestPoints` (engine map): between two profiling records builds
/// the query record (words `+8`, `+0x0C`, `+0x10` zero), a result point whose bodies (`+0x20`,
/// `+0x24`) are the two bodies, and asks `calculateClosestPoint`; when it reports a point the
/// collector's (`collector`, slot 4) is called with it.
pub fn hkp_box_box_agent_static_get_closest_points(
    e: &mut Engine,
    body_a: u32,
    body_b: u32,
    input: u32,
    collector: u32,
) {
    e.with_stack(0x1c8, |e, frame| {
        let f = frame.addr();
        profile_record(e, PROFILE_BEGIN);
        let query_frame = QueryFrame {
            transform: f + 0x58,
            extents_a: f + 0x18,
            extents_b: f + 8,
            query: f + 0x98,
        };
        build_query(e, &query_frame, body_a, body_b, input);
        let query = query_frame.query;
        e.mem.set_u32(query + 8, 0);
        e.mem.set_u32(query + 0xc, 0);
        e.mem.set_u32(query + 0x10, 0);
        let point = f + 0x28;
        e.mem.set_u32(point + 0x20, body_a);
        e.mem.set_u32(point + 0x24, body_b);
        e.call(CALCULATE_CLOSEST_POINT, &args![query, f + 3, point]);
        if e.mem.u8(f + 3) != 0 {
            e.vcall(collector, 4, &args![point]);
        }
        profile_record(e, PROFILE_END);
    });
}

// Translated from 00d406a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxBoxAgent::getClosestPoints` (engine map): passes its four words on to
/// `staticGetClosestPoints`.
pub fn hkp_box_box_agent_get_closest_points(
    e: &mut Engine,
    body_a: u32,
    body_b: u32,
    input: u32,
    collector: u32,
) {
    hkp_box_box_agent_static_get_closest_points(e, body_a, body_b, input, collector);
}

// Translated from 00d406c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxBoxAgent::staticGetPenetrations` (engine map): between two profiling records builds
/// the query record (words `+8`, `+0x0C`, `+0x10` zero), prepares it (`00d40940`) and runs
/// `checkIntersection` on it with the tolerance vector at `+0x80`; when that returns 0 the
/// collector's (`collector`, slot 4) is called with the two bodies.
pub fn hkp_box_box_agent_static_get_penetrations(
    e: &mut Engine,
    body_a: u32,
    body_b: u32,
    input: u32,
    collector: u32,
) {
    e.with_stack(0x194, |e, frame| {
        let f = frame.addr();
        profile_record(e, PROFILE_BEGIN);
        let query_frame = QueryFrame {
            transform: f + 0x24,
            extents_a: f + 0x14,
            extents_b: f + 4,
            query: f + 0x64,
        };
        build_query(e, &query_frame, body_a, body_b, input);
        let query = query_frame.query;
        e.mem.set_u32(query + 8, 0);
        e.mem.set_u32(query + 0xc, 0);
        e.mem.set_u32(query + 0x10, 0);
        e.call(PREPARE_QUERY, &args![query]);
        let intersecting = e
            .call(CHECK_INTERSECTION, &args![query, query + 0x80])
            .u32();
        if intersecting == 0 {
            e.vcall(collector, 4, &args![body_a, body_b]);
        }
        profile_record(e, PROFILE_END);
    });
}

// Translated from 00d408c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxBoxAgent::getPenetrations` (engine map): passes its four words on to
/// `staticGetPenetrations`.
pub fn hkp_box_box_agent_get_penetrations(
    e: &mut Engine,
    body_a: u32,
    body_b: u32,
    input: u32,
    collector: u32,
) {
    hkp_box_box_agent_static_get_penetrations(e, body_a, body_b, input, collector);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_0180, hkp_box_box_agent_cleanup(Ptr, u32)),
        entry!(
            0x00d4_0280,
            hkp_box_box_agent_process_collision(Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x00d4_0470,
            hkp_box_box_agent_static_get_closest_points(u32, u32, u32, u32)
        ),
        entry!(
            0x00d4_06a0,
            hkp_box_box_agent_get_closest_points(u32, u32, u32, u32)
        ),
        entry!(
            0x00d4_06c0,
            hkp_box_box_agent_static_get_penetrations(u32, u32, u32, u32)
        ),
        entry!(
            0x00d4_08c0,
            hkp_box_box_agent_get_penetrations(u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const OWNER_SLOT: u32 = 0x0900_0001;
    const DELETE_SLOT: u32 = 0x0900_0002;
    const COLLECTOR_SLOT: u32 = 0x0900_0003;

    fn bits(v: Vec4) -> [u32; 4] {
        v.map(f32::to_bits)
    }

    /// An engine with the exe's data range mapped and a profiling stream (room for 4
    /// records at the returned address, cursor at `+4`, limit at `+0xC`).
    fn setup() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let records = e.mem.alloc(0x30);
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x30);
        e.set_global(PROFILE_SLOT, 5u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        (e, log, stream)
    }

    fn log_callees(e: &mut Engine, log: &Log, addresses: &[u32]) {
        for &address in addresses {
            let sink = Rc::clone(log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The labels of the profiling records written so far.
    fn labels(e: &Engine, stream: u32) -> Vec<u32> {
        let first = e.mem.u32(stream + 0xc) - 0x30;
        let cursor = e.mem.u32(stream + 4);
        (first..cursor).step_by(12).map(|a| e.mem.u32(a)).collect()
    }

    #[test]
    fn cleanup_calls_the_owner_for_every_id_then_deletes_the_agent() {
        let (mut e, log, _) = setup();
        log_callees(&mut e, &log, &[OWNER_SLOT, DELETE_SLOT]);
        let owner_table = e.mem.alloc(0x20);
        e.mem.set_u32(owner_table + 0x14, OWNER_SLOT);
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, owner_table);
        let agent_table = e.mem.alloc(8);
        e.mem.set_u32(agent_table, DELETE_SLOT);
        let agent = e.mem.alloc(0x40);
        e.mem.set_u32(agent, agent_table);
        e.mem.set_u32(agent + 8, owner);
        e.mem.set_u8(agent + 0x31, 2);
        e.mem.set_u16(agent + 0x12, 0x1111);
        e.mem.set_u16(agent + 0x16, 0x2222);
        e.call(0x00d4_0180, &args![agent, 0x77u32]);
        assert_eq!(
            calls_to(&log, OWNER_SLOT),
            vec![vec![owner, 0x1111, 0x77], vec![owner, 0x2222, 0x77]]
        );
        assert_eq!(calls_to(&log, DELETE_SLOT), vec![vec![agent, 1]]);
    }

    #[test]
    fn cleanup_without_entries_only_deletes_the_agent() {
        let (mut e, log, _) = setup();
        log_callees(&mut e, &log, &[OWNER_SLOT, DELETE_SLOT]);
        let agent_table = e.mem.alloc(8);
        e.mem.set_u32(agent_table, DELETE_SLOT);
        let agent = e.mem.alloc(0x40);
        e.mem.set_u32(agent, agent_table);
        e.call(0x00d4_0180, &args![agent, 0x77u32]);
        assert!(calls_to(&log, OWNER_SLOT).is_empty());
        assert_eq!(calls_to(&log, DELETE_SLOT), vec![vec![agent, 1]]);
    }

    /// Two boxes: body A (radius 0.5, half extents (1, 2, 3, 4)), body B (radius 0.25, half
    /// extents (5, 6, 7, 8)), tolerance 0.125, and a transform double that fills its output
    /// with the vectors `(n, n + 1, n + 2, n + 3)` for `n` = 10, 20, 30, 40.
    fn scene(e: &mut Engine, log: &Log) -> (u32, u32, u32, u32) {
        e.set_global(QUERY_CONSTANT, 0.01f32);
        let body = |e: &mut Engine, radius: f32, half: Vec4, transform: u32| {
            let shape = e.mem.alloc(0x30);
            e.mem.set_f32(shape + 0x10, radius);
            store_vec(e, shape + 0x20, half);
            let body = e.mem.alloc(0x10);
            e.mem.set_u32(body, shape);
            e.mem.set_u32(body + 8, transform);
            body
        };
        let body_a = body(e, 0.5, [1.0, 2.0, 3.0, 4.0], 0x6000);
        let body_b = body(e, 0.25, [5.0, 6.0, 7.0, 8.0], 0x7000);
        let input = e.mem.alloc(0x10);
        e.mem.set_f32(input + 8, 0.125);
        let sink = Rc::clone(log);
        e.register_double(SET_MUL_INVERSE_MUL, move |e, a| {
            sink.borrow_mut().push((SET_MUL_INVERSE_MUL, a.to_vec()));
            for row in 0..4u32 {
                let n = 10.0 * (row + 1) as f32;
                store_vec(e, a[0] + 0x10 * row, [n, n + 1.0, n + 2.0, n + 3.0]);
            }
            Ret::default()
        });
        let collector_table = e.mem.alloc(0x10);
        e.mem.set_u32(collector_table + 4, COLLECTOR_SLOT);
        let collector = e.mem.alloc(8);
        e.mem.set_u32(collector, collector_table);
        log_callees(e, log, &[COLLECTOR_SLOT]);
        (body_a, body_b, input, collector)
    }

    /// The query record's 0xb8 bytes as words.
    fn snapshot(e: &Engine, query: u32) -> Vec<u32> {
        (0..0x2e).map(|i| e.mem.u32(query + 4 * i)).collect()
    }

    /// Checks the words every work function leaves in the query record.
    fn check_query(words: &[u32], body_a: u32, body_b: u32) {
        assert_eq!(words[0], body_a);
        assert_eq!(words[1], body_b);
        assert_eq!(words[5], 0x6000);
        assert_eq!(words[6], 0x7000);
        for (row, n) in [10.0f32, 20.0, 30.0, 40.0].into_iter().enumerate() {
            assert_eq!(
                words[8 + 4 * row..12 + 4 * row],
                bits([n, n + 1.0, n + 2.0, n + 3.0])
            );
        }
        assert_eq!(words[0x18..0x1c], bits([1.5, 2.5, 3.5, 4.5]));
        assert_eq!(words[0x1c..0x20], bits([5.25, 6.25, 7.25, 8.25]));
        assert_eq!(words[0x20..0x24], bits([0.125; 4]));
        assert_eq!(words[0x24..0x28], bits([1.625, 2.625, 3.625, 4.625]));
        assert_eq!(words[0x28..0x2c], bits([5.375, 6.375, 7.375, 8.375]));
        assert_eq!(words[0x2c], 0.125f32.to_bits());
        assert_eq!(words[0x2d], 0.01f32.to_bits());
    }

    #[test]
    fn closest_points_reports_the_point_when_the_detection_finds_one() {
        let (mut e, log, stream) = setup();
        let (body_a, body_b, input, collector) = scene(&mut e, &log);
        let record = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&record);
        let sink_log = Rc::clone(&log);
        e.register_double(CALCULATE_CLOSEST_POINT, move |e, a| {
            sink_log
                .borrow_mut()
                .push((CALCULATE_CLOSEST_POINT, a.to_vec()));
            *sink.borrow_mut() = snapshot(e, a[0]);
            e.mem.set_u8(a[1], 1);
            e.mem.set_u32(a[2], 0xfeed);
            Ret::default()
        });
        let point_bodies = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&point_bodies);
        e.register_double(COLLECTOR_SLOT, move |e, a| {
            *sink.borrow_mut() = vec![
                e.mem.u32(a[1]),
                e.mem.u32(a[1] + 0x20),
                e.mem.u32(a[1] + 0x24),
            ];
            Ret::default()
        });
        e.call(0x00d4_0470, &args![body_a, body_b, input, collector]);
        let words = record.borrow();
        check_query(&words, body_a, body_b);
        assert_eq!(words[2..5], [0, 0, 0]);
        assert_eq!(*point_bodies.borrow(), vec![0xfeed, body_a, body_b]);
        assert_eq!(labels(&e, stream), vec![PROFILE_BEGIN, PROFILE_END]);
        assert_eq!(
            calls_to(&log, SET_MUL_INVERSE_MUL)[0][1..],
            [0x6000, 0x7000]
        );
    }

    #[test]
    fn closest_points_without_a_point_does_not_call_the_collector() {
        let (mut e, log, stream) = setup();
        let (body_a, body_b, input, collector) = scene(&mut e, &log);
        log_callees(&mut e, &log, &[CALCULATE_CLOSEST_POINT]);
        e.call(0x00d4_06a0, &args![body_a, body_b, input, collector]);
        assert!(calls_to(&log, COLLECTOR_SLOT).is_empty());
        assert_eq!(calls_to(&log, CALCULATE_CLOSEST_POINT).len(), 1);
        assert_eq!(labels(&e, stream), vec![PROFILE_BEGIN, PROFILE_END]);
    }

    #[test]
    fn penetrations_report_the_pair_when_the_boxes_do_not_intersect() {
        let (mut e, log, stream) = setup();
        let (body_a, body_b, input, collector) = scene(&mut e, &log);
        log_callees(&mut e, &log, &[PREPARE_QUERY]);
        let record = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&record);
        let sink_log = Rc::clone(&log);
        e.register_double(CHECK_INTERSECTION, move |e, a| {
            sink_log.borrow_mut().push((CHECK_INTERSECTION, a.to_vec()));
            *sink.borrow_mut() = snapshot(e, a[0]);
            Ret::default()
        });
        e.call(0x00d4_08c0, &args![body_a, body_b, input, collector]);
        let words = record.borrow();
        check_query(&words, body_a, body_b);
        assert_eq!(words[2..5], [0, 0, 0]);
        let check = calls_to(&log, CHECK_INTERSECTION);
        assert_eq!(check[0][1], check[0][0] + 0x80);
        assert_eq!(calls_to(&log, PREPARE_QUERY), vec![vec![check[0][0]]]);
        assert_eq!(
            calls_to(&log, COLLECTOR_SLOT),
            vec![vec![collector, body_a, body_b]]
        );
        assert_eq!(labels(&e, stream), vec![PROFILE_BEGIN, PROFILE_END]);
    }

    #[test]
    fn penetrations_stay_silent_when_the_boxes_intersect() {
        let (mut e, log, _) = setup();
        let (body_a, body_b, input, collector) = scene(&mut e, &log);
        log_callees(&mut e, &log, &[PREPARE_QUERY]);
        e.register(CHECK_INTERSECTION, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call(0x00d4_06c0, &args![body_a, body_b, input, collector]);
        assert!(calls_to(&log, COLLECTOR_SLOT).is_empty());
    }

    #[test]
    fn process_collision_runs_the_manifold_on_the_query_between_profile_records() {
        let (mut e, log, stream) = setup();
        let (body_a, body_b, input, _) = scene(&mut e, &log);
        let record = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&record);
        let sink_log = Rc::clone(&log);
        e.register_double(CALC_MANIFOLD, move |e, a| {
            sink_log.borrow_mut().push((CALC_MANIFOLD, a.to_vec()));
            *sink.borrow_mut() = snapshot(e, a[0]);
            Ret::default()
        });
        let agent = e.mem.alloc(0x40);
        e.mem.set_u32(agent + 8, 0xabcd);
        e.call(0x00d4_0280, &args![agent, body_a, body_b, input, 0x4444u32]);
        let words = record.borrow();
        check_query(&words, body_a, body_b);
        assert_eq!(words[2..5], [input, 0xabcd, 0x4444]);
        assert_eq!(calls_to(&log, CALC_MANIFOLD)[0][1], agent + 0x10);
        assert_eq!(labels(&e, stream), vec![PROFILE_BEGIN, PROFILE_END]);
    }
}
