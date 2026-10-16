//! `Havok SDK/hkpconvexlistshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `TlsGetValue` import slot (`kernel32`).
const TLS_GET_VALUE: u32 = 0x00fd_f044;
/// The TLS index of the profiling record stream (a word in `.data`).
const PROFILE_SLOT_0126835C: u32 = 0x0126_835c;
/// The record labels: `"TtrcCxList"` and the end label shared with other
/// Havok profiling scopes.
const CX_LIST_LABEL: u32 = 0x010d_1094;
const END_LABEL: u32 = 0x010c_41f8;

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low_00d5e500() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a profiling record (`label`, time stamp) to the thread's record
/// stream (`TlsGetValue` of the index at `0126835c`) while its cursor
/// (`+4`) is below its limit (`+0xc`); a record is 12 bytes.
fn profile_record_00d5e500(e: &mut Engine, label: u32) {
    let slot: u32 = e.global(PROFILE_SLOT_0126835C);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low_00d5e500());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

// Translated from 00d5e500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListShape::castRayImpl` (map name). Thiscall, three words
/// (`RET 0xc`); returns `result`. Writes a profiling record, counts a level
/// in the collector (`+0x40`), and walks the child shapes with the iterator
/// embedded at `this + 0x14` (virtual slots `+8` first key, `+0xc` next
/// key, `+0x14` child for a key and a 0x200-byte scratch buffer; `-1`
/// ends). Each child's slot `+0x20` is called with a one-byte output,
/// `input` and `collector`; the key of the last child that reports a hit is
/// kept. The level is then popped; a kept key is stored at
/// `collector + 0x20 + level * 4` and `result` gets whether there was one.
/// `RDTSC` is the host clock.
pub fn hkp_convex_list_shape_cast_ray_impl(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    input: Ptr,
    collector: Ptr,
) -> Ptr {
    profile_record_00d5e500(e, CX_LIST_LABEL);
    let collector_a = collector.addr();
    let level = e.mem.u32(collector_a + 0x40).wrapping_add(1);
    e.mem.set_u32(collector_a + 0x40, level);
    let iterator = this.addr() + 0x14;
    let mut best_key = u32::MAX;
    e.with_stack(0x214, |e, frame| {
        let byte = frame.addr() + 0x1f;
        let buffer = frame.addr() + 0x20;
        let mut key = e.vcall(iterator, 8, &args![]).u32();
        while key != u32::MAX {
            let child = e.vcall(iterator, 0x14, &args![key, buffer]).u32();
            e.vcall(child, 0x20, &args![byte, input, collector]);
            if e.mem.u8(byte) != 0 {
                best_key = key;
            }
            key = e.vcall(iterator, 0xc, &args![key]).u32();
        }
    });
    let level = e.mem.u32(collector_a + 0x40).wrapping_sub(1);
    e.mem.set_u32(collector_a + 0x40, level);
    if best_key != u32::MAX {
        e.mem
            .set_u32(collector_a + 0x20 + level.wrapping_mul(4), best_key);
    }
    e.mem.set_u8(result.addr(), (best_key != u32::MAX) as u8);
    profile_record_00d5e500(e, END_LABEL);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d5e500, hkp_convex_list_shape_cast_ray_impl(Ptr, Ptr, Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    type CastLog = Rc<RefCell<Vec<Vec<u32>>>>;

    const ITERATOR_FIRST: u32 = 0x0090_5000;
    const ITERATOR_NEXT: u32 = 0x0090_5100;
    const ITERATOR_CHILD: u32 = 0x0090_5200;
    const CHILD_CAST: u32 = 0x0090_5300;

    /// A list shape whose iterator walks keys 0, 1, 2 (then -1); the child
    /// of key `k` reports a hit when `hits[k]`. Returns (engine, this,
    /// collector, record stream, log of the cast calls).
    fn shape_00d5e500(hits: [bool; 3]) -> (Engine, u32, u32, u32, CastLog) {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        let stream = e.mem.alloc(0x40);
        let records = e.mem.alloc(0x30);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x30);
        e.set_global(PROFILE_SLOT_0126835C, 5u32);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a, [5]);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        let mut slots = vec![0u32; 8];
        slots[2] = ITERATOR_FIRST;
        slots[3] = ITERATOR_NEXT;
        slots[5] = ITERATOR_CHILD;
        e.put_vtable(0x0090_2000, &slots);
        let mut child_slots = vec![0u32; 9];
        child_slots[8] = CHILD_CAST;
        e.put_vtable(0x0090_3000, &child_slots);
        let children: Vec<u32> = (0..3)
            .map(|_| {
                let child = e.mem.alloc(8);
                e.mem.set_u32(child, 0x0090_3000);
                child
            })
            .collect();
        e.register(ITERATOR_FIRST, |_, _| Ret::default());
        e.register(ITERATOR_NEXT, |_, a| Ret {
            eax: if a[1] >= 2 { u32::MAX } else { a[1] + 1 },
            ..Ret::default()
        });
        e.register_double(ITERATOR_CHILD, move |_, a| Ret {
            eax: children[a[1] as usize],
            ..Ret::default()
        });
        let casts = Rc::new(RefCell::new(Vec::new()));
        let cast_log = Rc::clone(&casts);
        e.register_double(CHILD_CAST, move |e, a| {
            let index = cast_log.borrow().len();
            cast_log.borrow_mut().push(a.to_vec());
            e.mem.set_u8(a[1], hits[index] as u8);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, 0x0090_2000);
        let collector = e.mem.alloc(0x60);
        e.mem.set_u32(collector + 0x40, 2);
        (e, this, collector, stream, casts)
    }

    #[test]
    fn hkp_convex_list_shape_cast_ray_impl_keeps_the_last_hitting_key() {
        let (mut e, this, collector, stream, casts) = shape_00d5e500([true, false, true]);
        let result = e.mem.alloc(4);
        let input = e.mem.alloc(0x20);
        let back = e.call(
            0x00d5_e500,
            &args![
                Ptr::<()>::new(this),
                Ptr::<()>::new(result),
                Ptr::<()>::new(input),
                Ptr::<()>::new(collector)
            ],
        );
        assert_eq!(back.u32(), result);
        assert_eq!(e.mem.u8(result), 1);
        // Level 2 was pushed and popped; the key of the last hit (2) is
        // stored in the slot of the popped level.
        assert_eq!(e.mem.u32(collector + 0x40), 2);
        assert_eq!(e.mem.u32(collector + 0x20 + 2 * 4), 2);
        let casts = casts.borrow();
        assert_eq!(casts.len(), 3);
        assert_eq!(casts[0][2..], [input, collector]);
        // Two profile records: the scope label and the end label.
        let records = e.mem.u32(stream + 4);
        assert_eq!(records, e.mem.u32(stream + 0xc) - 0x30 + 24);
        let first = records - 24;
        assert_eq!(e.mem.u32(first), CX_LIST_LABEL);
        assert_eq!(e.mem.u32(first + 12), END_LABEL);
    }

    #[test]
    fn hkp_convex_list_shape_cast_ray_impl_reports_no_hit() {
        let (mut e, this, collector, _, _) = shape_00d5e500([false, false, false]);
        let result = e.mem.alloc(4);
        e.mem.set_u8(result, 0xee);
        e.mem.set_u32(collector + 0x20 + 2 * 4, 0x1234);
        let input = e.mem.alloc(0x20);
        e.call(
            0x00d5_e500,
            &args![
                Ptr::<()>::new(this),
                Ptr::<()>::new(result),
                Ptr::<()>::new(input),
                Ptr::<()>::new(collector)
            ],
        );
        assert_eq!(e.mem.u8(result), 0);
        assert_eq!(e.mem.u32(collector + 0x20 + 2 * 4), 0x1234);
        assert_eq!(e.mem.u32(collector + 0x40), 2);
    }
}
