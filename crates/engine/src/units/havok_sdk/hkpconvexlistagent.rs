//! `Havok SDK/hkpconvexlistagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TlsGetValue` import slot and the TLS index of Havok's monitor (timer) stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// `hkpIterativeLinearCastAgent::staticLinearCast` (Xbox PDB, `00daf8e0`, cdecl `(p0, p1, p2,
/// collector, collector)`).
const ITERATIVE_LINEAR_CAST: u32 = 0x00da_f8e0;
/// `00d43b90` (cdecl, five words): the cast `00d34810` forwards to.
const CAST_FORWARD: u32 = 0x00d4_3b90;
/// Timer labels: the start pair, the second-stage label and the end label.
const LABEL_LINEAR_CAST: u32 = 0x010c_ec54;
const LABEL_LINEAR_CAST_DETAIL: u32 = 0x010c_ec34;
const LABEL_CONVEX_CAST: u32 = 0x010c_ec4c;
const LABEL_END: u32 = 0x0107_6f38;
/// The vtable of the stack collector the first stage builds (`010cebe0`) and the one the
/// forwarding stage builds (`010ce80c`).
const FIRST_COLLECTOR_VTABLE: u32 = 0x010c_ebe0;
const CAST_COLLECTOR_VTABLE: u32 = 0x010c_e80c;
/// The `float` both collectors start with (the bits `0x7f7fffee`).
const FAR_DISTANCE: u32 = 0x0104_da48;
/// The sign mask (`010c44d0`, four lanes) that `XORPS` negates a vector with.
const SIGN_MASK: u32 = 0x010c_44d0;

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

/// The `float` constant `FLD`/`FSTP` copied: a signalling NaN comes back quiet.
fn far_distance_bits(e: &Engine) -> u32 {
    x87_quiet_bits(e.mem.u32(FAR_DISTANCE))
}

// Translated from 00d34810 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cast the convex-list agent forwards to `00d43b90`: copies the 0x70-byte cast input
/// (`input`) with the vector at +0x50 negated lane by lane, builds one collector on the
/// stack (the vtable `010ce80c`, the far distance, a copy of the input vector at +0x10 and
/// `cast_collector` at +0x20) and, when `start_collector` is non-zero, a second one of the same
/// shape holding `start_collector`; then calls `00d43b90(second, first, copy, collector,
/// second collector or 0)`. The parameter roles are described from their use.
pub fn fn_00d34810(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    // The exe's frame, 0xD8 bytes: the first collector at +8, the second at +0x38 and the
    // input copy at +0x68.
    e.with_stack(0xd8, |e, frame| {
        let frame = frame.addr();
        let far = far_distance_bits(e);
        e.mem.set_u32(frame + 0xc, far);
        let input_bytes = e.mem.bytes(input.addr(), 0x70);
        e.mem.write(frame + 0x68, &input_bytes);
        for lane in 0..4u32 {
            let word = e.mem.u32(input.addr() + 0x50 + 4 * lane);
            e.mem.set_u32(frame + 0x18 + 4 * lane, word);
        }
        e.mem.set_u32(frame + 0x28, cast_collector);
        for lane in 0..4u32 {
            let word = e.mem.u32(input.addr() + 0x50 + 4 * lane);
            let mask = e.mem.u32(SIGN_MASK + 4 * lane);
            e.mem.set_u32(frame + 0xb8 + 4 * lane, word ^ mask);
        }
        e.mem.set_u32(frame + 8, CAST_COLLECTOR_VTABLE);
        let second_collector = if start_collector != 0 {
            e.mem.set_u32(frame + 0x38, CAST_COLLECTOR_VTABLE);
            e.mem.set_u32(frame + 0x3c, far);
            for lane in 0..4u32 {
                let word = e.mem.u32(input.addr() + 0x50 + 4 * lane);
                e.mem.set_u32(frame + 0x48 + 4 * lane, word);
            }
            e.mem.set_u32(frame + 0x58, start_collector);
            frame + 0x38
        } else {
            0
        };
        e.call(
            CAST_FORWARD,
            &args![second, first, frame + 0x68, frame + 8, second_collector],
        );
    });
}

// Translated from 00d33ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The timed linear cast of the convex-list agent (the exe has a `linearCast` virtual that
/// forwards here): starts a timer record, builds a collector on the stack (the vtable
/// `010cebe0`, the far distance at +4 and +0x2C, a flag byte at +8 cleared) and runs the
/// iterative cast `00daf8e0(first, second, input, collector, collector)` on it; when the
/// collector's flag was set, a second timer record is written and the cast is forwarded
/// with `00d34810`. A closing record ends it. The parameter roles are described from their
/// use.
pub fn fn_00d33ca0(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    monitor_start_record(e, LABEL_LINEAR_CAST, LABEL_LINEAR_CAST_DETAIL);
    // The exe's frame: the collector at +0x10 (its field at +0x2C is the frame's +0x3C).
    e.with_stack(0x40, |e, frame| {
        let collector = frame.addr() + 0x10;
        let far = far_distance_bits(e);
        e.mem.set_u32(collector + 0x2c, far);
        e.mem.set_u32(collector + 4, far);
        e.mem.set_u32(collector, FIRST_COLLECTOR_VTABLE);
        e.mem.set_u8(collector + 8, 0);
        e.call(
            ITERATIVE_LINEAR_CAST,
            &args![first, second, input, collector, collector],
        );
        if e.mem.u8(collector + 8) != 0 {
            monitor_record(e, LABEL_CONVEX_CAST);
            fn_00d34810(e, first, second, input, cast_collector, start_collector);
        }
    });
    monitor_record(e, LABEL_END);
}

// Translated from 00d33da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListAgent::linearCast` (Xbox PDB): forwards its five words to `00d33ca0`.
pub fn hkp_convex_list_agent_linear_cast(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    fn_00d33ca0(e, first, second, input, cast_collector, start_collector);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d3_3ca0, fn_00d33ca0(Ptr, Ptr, Ptr, u32, u32)),
        entry!(
            0x00d3_3da0,
            hkp_convex_list_agent_linear_cast(Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(0x00d3_4810, fn_00d34810(Ptr, Ptr, Ptr, u32, u32)),
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

    /// What the forwarding double saw: its arguments, the input copy, both collectors.
    #[derive(Debug, Default, Clone, PartialEq)]
    struct Forwarded {
        args: Vec<u32>,
        copy: Vec<u8>,
        first: Vec<u32>,
        second: Vec<u32>,
    }

    type Seen = Rc<RefCell<Vec<Forwarded>>>;

    fn rig() -> (Engine, Seen) {
        let mut e = test_engine();
        e.set_global(FAR_DISTANCE, 0x7f7f_ffeeu32);
        for lane in 0..4 {
            e.set_global(SIGN_MASK + 4 * lane, 0x8000_0000u32);
        }
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(CAST_FORWARD, move |e, a| {
            let words = |address: u32, count: u32| -> Vec<u32> {
                (0..count).map(|i| e.mem.u32(address + 4 * i)).collect()
            };
            let second = if a[4] != 0 {
                words(a[4], 9)
            } else {
                Vec::new()
            };
            sink.borrow_mut().push(Forwarded {
                args: a.to_vec(),
                copy: e.mem.bytes(a[2], 0x70),
                first: words(a[3], 9),
                second,
            });
            Ret::default()
        });
        (e, seen)
    }

    fn input(e: &mut Engine) -> u32 {
        let input = e.mem.alloc(0x70);
        for i in 0..0x1c {
            e.mem.set_u32(input + 4 * i, 0x100 + i);
        }
        input
    }

    #[test]
    fn forwarding_cast_builds_one_or_two_collectors_and_negates_the_vector() {
        let (mut e, seen) = rig();
        let input = input(&mut e);
        e.call(
            0x00d3_4810,
            &args![0x11u32, 0x22u32, input, 0x33u32, 0x44u32],
        );
        e.call(0x00d3_4810, &args![0x11u32, 0x22u32, input, 0x33u32, 0u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        let with_start = &seen[0];
        assert_eq!(with_start.args[0..2], [0x22, 0x11]);
        // The copy equals the input except for the negated vector at +0x50.
        let mut expected = e.mem.bytes(input, 0x70);
        for lane in 0..4 {
            let at = 0x50 + 4 * lane;
            let word = u32::from_le_bytes(expected[at..at + 4].try_into().unwrap());
            expected[at..at + 4].copy_from_slice(&(word ^ 0x8000_0000).to_le_bytes());
        }
        assert_eq!(with_start.copy, expected);
        // Collector: vtable, far distance, ..., the vector at +0x10, the collector at +0x20.
        let collector = [
            CAST_COLLECTOR_VTABLE,
            0x7f7f_ffee,
            0,
            0,
            0x114,
            0x115,
            0x116,
            0x117,
            0x33,
        ];
        assert_eq!(with_start.first, collector);
        let mut second = collector;
        second[8] = 0x44;
        assert_eq!(with_start.second, second);
        assert_eq!(seen[1].args[4], 0);
        assert_eq!(seen[1].first, collector);
    }

    #[test]
    fn convex_cast_forwards_only_when_the_collector_flag_is_set() {
        let (mut e, seen) = rig();
        e.set_global(MONITOR_SLOT, 3u32);
        let stream = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x100);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        let hit = Rc::new(RefCell::new(false));
        let flag = Rc::clone(&hit);
        let collector_seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&collector_seen);
        e.register_double(ITERATIVE_LINEAR_CAST, move |e, a| {
            assert_eq!(a[3], a[4]);
            sink.borrow_mut()
                .push((e.mem.u32(a[3]), e.mem.u32(a[3] + 4), e.mem.u32(a[3] + 0x2c)));
            e.mem.set_u8(a[3] + 8, u8::from(*flag.borrow()));
            Ret::default()
        });
        let input = input(&mut e);
        e.call(0x00d3_3ca0, &args![1u32, 2u32, input, 3u32, 4u32]);
        assert!(seen.borrow().is_empty());
        // Start record (16 bytes) and the end record (12 bytes).
        assert_eq!(e.mem.u32(stream + 4), records + 0x10 + 0xc);
        assert_eq!(e.mem.u32(records), LABEL_LINEAR_CAST);
        assert_eq!(e.mem.u32(records + 0xc), LABEL_LINEAR_CAST_DETAIL);
        assert_eq!(e.mem.u32(records + 0x10), LABEL_END);
        *hit.borrow_mut() = true;
        e.call(0x00d3_3da0, &args![1u32, 2u32, input, 3u32, 4u32]);
        assert_eq!(seen.borrow().len(), 1);
        assert_eq!(seen.borrow()[0].args[0..2], [2, 1]);
        assert_eq!(
            collector_seen.borrow()[0],
            (FIRST_COLLECTOR_VTABLE, 0x7f7f_ffee, 0x7f7f_ffee)
        );
        // 16 + 12 + 16 + 12 + 12 bytes in all.
        assert_eq!(
            e.mem.u32(stream + 4),
            records + 0x10 + 0xc + 0x10 + 0xc + 0xc
        );
        assert_eq!(e.mem.u32(records + 0x1c + 0x10), LABEL_CONVEX_CAST);
    }
}
