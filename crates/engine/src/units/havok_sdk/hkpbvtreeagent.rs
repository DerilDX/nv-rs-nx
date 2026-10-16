//! `Havok SDK/hkpbvtreeagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TlsGetValue` import slot and the TLS index of Havok's monitor (timer) stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// The memory router (`00c85750`, no arguments): word +0x10 is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkTransform::setMulInverseMul` (Xbox PDB, `00cb22f0`, `this` = result, `(a, b)`).
const TRANSFORM_SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
/// A function (`004b4ab0`, `this` = a vector result, `(body motion, input + 0x50)`) that
/// produces the sweep displacement.
const SWEEP_DISPLACEMENT: u32 = 0x004b_4ab0;
/// Timer labels: `"LtBvTree"`, `"StQueryTree"`, `"StNarrowPhase"` and the end label.
const LABEL_BV_TREE: u32 = 0x010c_ea04;
const LABEL_QUERY_TREE: u32 = 0x010c_e7e0;
const LABEL_NARROW_PHASE: u32 = 0x010c_e9f4;
const LABEL_END: u32 = 0x0107_6f38;

/// Low word of the time-stamp counter (`RDTSC`); the engine has no model of it and the host
/// clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// The thread's monitor stream (`TlsGetValue` of the index at `0126835c`).
fn monitor_stream(e: &mut Engine) -> u32 {
    let index: u32 = e.global(MONITOR_SLOT);
    e.call(TLS_GET_VALUE, &args![index]).u32()
}

/// Appends a 12-byte timer record `(label, time stamp)` while the stream's cursor (+4) is
/// below its limit (+0xC).
fn monitor_record(e: &mut Engine, label: u32) {
    let stream = monitor_stream(e);
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0xc);
    }
}

/// Appends a 16-byte start record: `first` at +0, the time stamp at +4 and `second` at +0xC.
fn monitor_start_record(e: &mut Engine, first: u32, second: u32) {
    let stream = monitor_stream(e);
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, first);
        e.mem.set_u32(cursor + 0xc, second);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0x10);
    }
}

/// The four lanes of the vector at `addr`.
fn load4(e: &Engine, addr: u32) -> [f32; 4] {
    [0, 4, 8, 12].map(|offset| e.mem.f32(addr + offset))
}

/// `MINPS dest, src` per lane: `dest` when `dest < src`, otherwise `src`.
fn min_lane(dest: f32, src: f32) -> f32 {
    if dest < src {
        dest
    } else {
        src
    }
}

/// `MAXPS dest, src` per lane: `dest` when `dest > src`, otherwise `src`.
fn max_lane(dest: f32, src: f32) -> f32 {
    if dest > src {
        dest
    } else {
        src
    }
}

// Translated from 00d2f690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeAgent::calcAabbLinearCast` (Xbox PDB): asks the shape (`*first`, vtable slot
/// 0x1C) for the AABB of the cast, using the relative transform of the two bodies' motions
/// (`00cb22f0`) and the float at `input + 8`, writing it to the `aabb` (min at +0, max at
/// +0x10); then widens it by the sweep displacement `d` (`004b4ab0` with `input + 0x50`): the
/// minimum gains `min(0, d)` and the maximum `max(0, d)` in each of the four lanes.
pub fn hkp_bv_tree_agent_calc_aabb_linear_cast(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    aabb: Ptr,
) {
    let first_motion = e.mem.u32(first.addr() + 8);
    let second_motion = e.mem.u32(second.addr() + 8);
    let shape_owner = e.mem.u32(first.addr());
    e.with_stack(0x40, |e, transform| {
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![transform, second_motion, first_motion],
        );
        let distance = x87_quiet(e.mem.f32(input.addr() + 8));
        e.vcall(shape_owner, 0x1c, &args![transform, distance, aabb]);
    });
    e.with_stack(0x10, |e, displacement| {
        e.call(
            SWEEP_DISPLACEMENT,
            &args![displacement, second_motion, input.addr() + 0x50],
        );
        let moved = load4(e, displacement.addr());
        let lower = load4(e, aabb.addr());
        let upper = load4(e, aabb.addr() + 0x10);
        for lane in 0..4u32 {
            let d = moved[lane as usize];
            e.mem.set_f32(
                aabb.addr() + 4 * lane,
                min_lane(0.0, d) + lower[lane as usize],
            );
            e.mem.set_f32(
                aabb.addr() + 0x10 + 4 * lane,
                max_lane(0.0, d) + upper[lane as usize],
            );
        }
    });
}

// Translated from 00d30f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeAgent::staticLinearCast` (Xbox PDB): a timed cast of a body against a
/// bounding-volume tree. It computes the AABB of the cast, asks the tree shape (`*second`,
/// vtable slot 0x30) for the keys overlapping it into a stack array of 128 words, and for
/// each key gets the child shape (container slot 0x10 gives the container, whose slot 0x14
/// turns a key into a shape), builds a body record on the stack (the shape, the key, the
/// motion at `second + 8` and `second`) and calls the cast function found in the dispatcher
/// (`*input`) table at `0x9AC + 0x14 * type`, where the type is a byte of the shape's data at
/// `(*(*first + 0xC) + 0xD) * 0x20 + *input`. The array's heap storage, if it owns any, is
/// freed at the end. The parameter roles are described from their use.
pub fn hkp_bv_tree_agent_static_linear_cast(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    monitor_start_record(e, LABEL_BV_TREE, LABEL_QUERY_TREE);
    // The exe's stack frame: the AABB at +0x30, the key array at +0x50 (data, size,
    // capacity) with its inline storage at +0x5C, the body record at +0x20, and a scratch
    // buffer for child shapes at +0x260.
    e.with_stack(0x460, |e, frame| {
        let frame = frame.addr();
        hkp_bv_tree_agent_calc_aabb_linear_cast(e, first, second, input, Ptr::new(frame + 0x30));
        let shape_owner = e.mem.u32(second.addr());
        e.mem.set_u32(frame + 0x50, frame + 0x5c);
        e.mem.set_u32(frame + 0x54, 0);
        e.mem.set_u32(frame + 0x58, 0x8000_0080);
        e.vcall(shape_owner, 0x30, &args![frame + 0x30, frame + 0x50]);
        monitor_record(e, LABEL_NARROW_PHASE);
        let first_header = e.mem.u32(first.addr());
        let type_base = e.mem.u32(first_header + 0xc);
        let mut key_cursor = e.mem.u32(frame + 0x50);
        let key_end = key_cursor + e.mem.u32(frame + 0x54) * 4;
        e.mem.set_u32(frame + 0x28, e.mem.u32(second.addr() + 8));
        e.mem.set_u32(frame + 0x2c, second.addr());
        let container = e.vcall(shape_owner, 0x10, &args![]).u32();
        if key_cursor != key_end {
            let type_offset = type_base.wrapping_add(0xd) << 5;
            loop {
                let key = e.mem.u32(key_cursor);
                let shape = e.vcall(container, 0x14, &args![key, frame + 0x260]).u32();
                e.mem.set_u32(frame + 0x20, shape);
                e.mem.set_u32(frame + 0x24, e.mem.u32(key_cursor));
                let dispatcher = e.mem.u32(input.addr());
                let shape_data = e.mem.u32(shape + 0xc);
                let shape_type = e.mem.u8(shape_data
                    .wrapping_add(type_offset)
                    .wrapping_add(dispatcher));
                let function = e.mem.u32(dispatcher + 0x9ac + u32::from(shape_type) * 0x14);
                e.call(
                    function,
                    &args![first, frame + 0x20, input, cast_collector, start_collector],
                );
                key_cursor += 4;
                if key_cursor == key_end {
                    break;
                }
            }
        }
        monitor_record(e, LABEL_END);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(frame + 0x58);
        e.mem.set_u32(frame + 0x54, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(frame + 0x50);
            e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) * 4]);
        }
    });
}

// Translated from 00d315a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeAgent::linearCast` (Xbox PDB): forwards its five words to `staticLinearCast`
/// (`00d30f50`).
pub fn hkp_bv_tree_agent_linear_cast(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    hkp_bv_tree_agent_static_linear_cast(e, first, second, input, cast_collector, start_collector);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d2_f690,
            hkp_bv_tree_agent_calc_aabb_linear_cast(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d3_0f50,
            hkp_bv_tree_agent_static_linear_cast(Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x00d3_15a0,
            hkp_bv_tree_agent_linear_cast(Ptr, Ptr, Ptr, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A double at `address` that logs its arguments and answers `eax`.
    fn log_double(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// An object whose vtable has the given `(slot, double address)` entries.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for &(slot, target) in slots {
            e.mem.set_u32(vtable + slot, target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The monitor stream: TLS index 3, a cursor at `records` and room for `room` bytes.
    fn monitor(e: &mut Engine, room: u32) -> (u32, u32) {
        e.set_global(MONITOR_SLOT, 3u32);
        let stream = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + room);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        (stream, records)
    }

    #[test]
    fn calc_aabb_widens_the_box_by_the_sweep() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, TRANSFORM_SET_MUL_INVERSE_MUL, 0);
        e.register_double(0x0900_001c, |e, a| {
            // `this`, transform, float, aabb.
            assert_eq!(f32::from_bits(a[2]), 2.5);
            for lane in 0..4 {
                e.mem.set_f32(a[3] + 4 * lane, 1.0);
                e.mem.set_f32(a[3] + 0x10 + 4 * lane, 5.0);
            }
            Ret::default()
        });
        e.register_double(SWEEP_DISPLACEMENT, |e, a| {
            for (lane, v) in [1.0f32, -2.0, 0.0, -3.0].iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * lane as u32, *v);
            }
            Ret::default()
        });
        let owner = object(&mut e, &[(0x1c, 0x0900_001c)]);
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first, owner);
        e.mem.set_u32(first + 8, 0x111);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second + 8, 0x222);
        let input = e.mem.alloc(0x100);
        e.mem.set_f32(input + 8, 2.5);
        let aabb = e.mem.alloc(0x20);
        e.call(0x00d2_f690, &args![first, second, input, aabb]);
        let lower: Vec<f32> = (0..4).map(|i| e.mem.f32(aabb + 4 * i)).collect();
        let upper: Vec<f32> = (0..4).map(|i| e.mem.f32(aabb + 0x10 + 4 * i)).collect();
        assert_eq!(lower, vec![1.0, -1.0, 1.0, -2.0]);
        assert_eq!(upper, vec![6.0, 5.0, 5.0, 5.0]);
        let transform = &calls(&log, TRANSFORM_SET_MUL_INVERSE_MUL)[0];
        assert_eq!(transform[1..], [0x222, 0x111]);
    }

    #[test]
    fn static_linear_cast_calls_the_cast_function_of_each_overlapping_child() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let (stream, records) = monitor(&mut e, 0x100);
        // The geometry calls are not the subject here.
        log_double(&mut e, &log, TRANSFORM_SET_MUL_INVERSE_MUL, 0);
        log_double(&mut e, &log, SWEEP_DISPLACEMENT, 0);
        log_double(&mut e, &log, 0x0900_001c, 0);
        // The tree shape: slot 0x30 reports two keys in heap storage, slot 0x10 answers
        // the container, whose slot 0x14 answers a shape per key.
        let heap = e.mem.alloc(0x10);
        e.register_double(0x0900_0030, move |e, a| {
            e.mem.set_u32(heap, 7);
            e.mem.set_u32(heap + 4, 9);
            e.mem.set_u32(a[2], heap);
            e.mem.set_u32(a[2] + 4, 2);
            e.mem.set_u32(a[2] + 8, 4);
            Ret::default()
        });
        let shape_a = e.mem.alloc(0x20);
        let shape_b = e.mem.alloc(0x20);
        let first_header = object(&mut e, &[(0x1c, 0x0900_001c)]);
        let offset = 0xdu32 << 5;
        e.mem.set_u32(shape_a + 0xc, 0x10);
        e.mem.set_u32(shape_b + 0xc, 0x20);
        let dispatcher = e.mem.alloc(0xa80);
        let cast_one = 0x0900_0101;
        let cast_zero = 0x0900_0100;
        e.mem.set_u32(dispatcher + 0x9ac, cast_zero);
        e.mem.set_u32(dispatcher + 0x9ac + 0x14, cast_one);
        // The type byte of the first child is 1; the second child's byte stays 0.
        e.mem.set_u8(dispatcher + offset + 0x10, 1);
        log_double(&mut e, &log, cast_zero, 0);
        log_double(&mut e, &log, cast_one, 0);
        let container = object(&mut e, &[(0x14, 0x0900_0014)]);
        e.register_double(0x0900_0014, move |_, a| Ret {
            eax: if a[1] == 7 { shape_a } else { shape_b },
            ..Ret::default()
        });
        e.register_double(0x0900_0010, move |_, _| Ret {
            eax: container,
            ..Ret::default()
        });
        let owner = object(
            &mut e,
            &[
                (0x30, 0x0900_0030),
                (0x10, 0x0900_0010),
                (0x1c, 0x0900_001c),
            ],
        );
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first, first_header);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second, owner);
        e.mem.set_u32(second + 8, 0x222);
        let input = e.mem.alloc(0x100);
        e.mem.set_u32(input, dispatcher);
        // The heap array is released with its capacity in bytes.
        let router = e.mem.alloc(0x20);
        let allocator = object(&mut e, &[(0x10, 0x0900_0210)]);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        log_double(&mut e, &log, 0x0900_0210, 0);
        e.call(0x00d3_0f50, &args![first, second, input, 0xaau32, 0xbbu32]);
        let zero = calls(&log, cast_zero);
        let one = calls(&log, cast_one);
        assert_eq!((zero.len(), one.len()), (1, 1));
        assert_eq!(one[0][0], first);
        assert_eq!(one[0][2..], [input, 0xaa, 0xbb]);
        // The body record: shape, key, motion of the second body, the second body.
        let record = one[0][1];
        assert_eq!(e.mem.u32(record + 8), 0x222);
        assert_eq!(e.mem.u32(record + 0xc), second);
        assert_eq!(calls(&log, 0x0900_0210), vec![vec![allocator, heap, 16]]);
        // Three timer records were appended (16, 12 and 12 bytes).
        assert_eq!(e.mem.u32(stream + 4), records + 0x10 + 0xc + 0xc);
        assert_eq!(e.mem.u32(records), LABEL_BV_TREE);
        assert_eq!(e.mem.u32(records + 0xc), LABEL_QUERY_TREE);
        assert_eq!(e.mem.u32(records + 0x10), LABEL_NARROW_PHASE);
        assert_eq!(e.mem.u32(records + 0x1c), LABEL_END);
    }

    #[test]
    fn linear_cast_forwards_to_the_static_cast() {
        let mut e = test_engine();
        // A stream whose cursor is at its limit: no records are written.
        let (stream, records) = monitor(&mut e, 0);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, TRANSFORM_SET_MUL_INVERSE_MUL, 0);
        log_double(&mut e, &log, SWEEP_DISPLACEMENT, 0);
        log_double(&mut e, &log, 0x0900_001c, 0);
        log_double(&mut e, &log, 0x0900_0030, 0);
        log_double(&mut e, &log, 0x0900_0010, 0);
        let router = e.mem.alloc(0x20);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let owner = object(
            &mut e,
            &[
                (0x30, 0x0900_0030),
                (0x10, 0x0900_0010),
                (0x1c, 0x0900_001c),
            ],
        );
        let header = object(&mut e, &[(0x1c, 0x0900_001c)]);
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first, header);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second, owner);
        let input = e.mem.alloc(0x100);
        // The allocator the array would be freed through (never needed: inline storage).
        e.call(0x00d3_15a0, &args![first, second, input, 1u32, 2u32]);
        assert_eq!(calls(&log, 0x0900_0030).len(), 1);
        assert_eq!(calls(&log, 0x0900_0010).len(), 1);
        assert_eq!(e.mem.u32(stream + 4), records);
    }
}
