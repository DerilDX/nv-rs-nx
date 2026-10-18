//! `Havok SDK/run_00da0010` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00da0780 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x10-byte frame stores the vtable word 0x010d_460c at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x8 (argument 0) and
/// returns the word at the start of the frame. Nothing is destroyed.
pub fn fn_00da0780(e: &mut Engine) -> u32 {
    e.with_stack(0x10, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010d_460c);
        e.call(0x00cb_0a70, &args![base + 0x8, 0u32]);
        e.mem.u32(base)
    })
}

// Translated from 00da07d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x70-byte frame stores the vtable word 0x010d_461c at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x64 (argument 0) and
/// returns the word at the start of the frame. Nothing is destroyed.
pub fn fn_00da07d0(e: &mut Engine) -> u32 {
    e.with_stack(0x70, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010d_461c);
        e.call(0x00cb_0a70, &args![base + 0x64, 0u32]);
        e.mem.u32(base)
    })
}

// Translated from 00da0010 (decompiled, FalloutNV.exe 1.4.0.525)
/// A least-significant-digit radix sort of 8-byte elements `(key, value)` by their 32-bit key
/// (four passes of 8 bits, ping-ponging between the two arrays so that the result is in `data`
/// again): cdecl `(data, count, buffer)`. The exe's 0x2004-byte stack frame holds the four
/// 256-entry histograms (at `+0x1010`, `+0x1810`, `+0x1410` and `+0x1c10`, for key bytes 0, 1, 2
/// and 3) and the four tables of running destination pointers (`+0xc10` into `buffer`, `+0x810`
/// into `data`, `+0x410` into `buffer`, `+0x10` into `data`); it is allocated with `__chkstk`
/// (`00ec8660`, which only probes the stack pages and so has no counterpart here). Every loop is
/// unrolled by four in the exe: with `count > 0` each pass takes `((count - 1) >> 2) + 1` groups
/// of four elements, so a `count` that is not a multiple of 4 also counts and moves the (up to
/// three) elements after the end, as the exe does; nothing happens for `count <= 0`. The exe's
/// name for it is unknown (a `hkRadixSort`-style sort of `(key, value)` pairs).
pub fn fn_00da0010(e: &mut Engine, data: u32, count: i32, buffer: u32) {
    e.with_stack(0x2004, |e, frame| {
        let frame = frame.addr();
        // The histograms are zeroed first (four tables of 256 words).
        for table in [0x1010u32, 0x1810, 0x1410, 0x1c10] {
            for word in 0..0x100u32 {
                e.mem.set_u32(frame + table + 4 * word, 0);
            }
        }
        if count <= 0 {
            return;
        }
        // The groups of four elements of every pass.
        let groups = ((count as u32).wrapping_sub(1) >> 2).wrapping_add(1);
        let elements = groups.wrapping_mul(4);
        // Counting: the four key bytes of every element.
        let histograms = [0x1010u32, 0x1810, 0x1410, 0x1c10];
        for element in 0..elements {
            let key_at = data.wrapping_add(8 * element);
            for (byte, table) in histograms.iter().enumerate() {
                let digit = u32::from(e.mem.u8(key_at.wrapping_add(byte as u32)));
                let slot = frame + table + 4 * digit;
                let value = e.mem.u32(slot).wrapping_add(1);
                e.mem.set_u32(slot, value);
            }
        }
        // The first destination of each pass, then the running sums.
        let pointers = [0x0c10u32, 0x0810, 0x0410, 0x0010];
        let bases = [buffer, data, buffer, data];
        for (pointer_table, base) in pointers.iter().zip(bases) {
            e.mem.set_u32(frame + pointer_table, base);
        }
        for (count_table, pointer_table) in histograms.iter().zip(pointers) {
            for digit in 0..255u32 {
                let counted = e.mem.u32(frame + count_table + 4 * digit);
                let start = e.mem.u32(frame + pointer_table + 4 * digit);
                e.mem.set_u32(
                    frame + pointer_table + 4 * (digit + 1),
                    start.wrapping_add(counted.wrapping_mul(8)),
                );
            }
        }
        // The four passes: byte 0 from `data` into `buffer`, byte 1 back, byte 2 and byte 3.
        let passes = [
            (data, 0u32, pointers[0]),
            (buffer, 1, pointers[1]),
            (data, 2, pointers[2]),
            (buffer, 3, pointers[3]),
        ];
        for (source, byte, pointer_table) in passes {
            for element in 0..elements {
                let at = source.wrapping_add(8 * element);
                let key = e.mem.u32(at);
                let digit = u32::from(e.mem.u8(at.wrapping_add(byte)));
                let value = e.mem.u32(at.wrapping_add(4));
                let slot = frame + pointer_table + 4 * digit;
                let destination = e.mem.u32(slot);
                e.mem.set_u32(destination, key);
                e.mem.set_u32(destination.wrapping_add(4), value);
                e.mem.set_u32(slot, destination.wrapping_add(8));
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da0780, fn_00da0780() -> u32),
        entry!(0x00da07d0, fn_00da07d0() -> u32),
        entry!(0x00da_0010, fn_00da0010(u32, i32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What the probe saw on one call: `this`, the argument, the word `behind` bytes before `this`
    /// (the frame's vtable slot) and, when asked for, the `u32` at +0x20, `u16` at +0x24 and `u16`
    /// at +0x26 of the frame.
    type ProbeCall = (u32, u32, u32, [u32; 3]);
    type ProbeLog = Rc<RefCell<Vec<ProbeCall>>>;

    /// Double for the member constructor: logs, per call, what `ProbeCall` describes, with the
    /// vtable slot `behind[call]` bytes before `this`.
    fn frame_probe(e: &mut Engine, address: u32, behind: &[u32], with_tail: bool) -> ProbeLog {
        let log: ProbeLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        let behind = behind.to_vec();
        e.register_double(address, move |e, a| {
            let call = seen.borrow().len();
            let slot = a[0] - behind[call];
            let mut tail = [0; 3];
            if with_tail {
                tail = [
                    e.mem.u32(slot + 0x20),
                    u32::from(e.mem.u16(slot + 0x24)),
                    u32::from(e.mem.u16(slot + 0x26)),
                ];
            }
            seen.borrow_mut().push((a[0], a[1], e.mem.u32(slot), tail));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn fn_00da0780_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x8], false);
        let result = e.call(0x00da_0780, &args![]).u32();
        assert_eq!(result, 0x010d_460c);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010d_460c);
        }
    }

    #[test]
    fn fn_00da07d0_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x64], false);
        let result = e.call(0x00da_07d0, &args![]).u32();
        assert_eq!(result, 0x010d_461c);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010d_461c);
        }
    }
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    fn pairs(e: &mut Engine, items: &[(u32, u32)], padding: usize) -> u32 {
        let data = e.mem.alloc(8 * (items.len() + padding) as u32);
        for (i, (key, value)) in items.iter().enumerate() {
            e.mem.set_u32(data + 8 * i as u32, *key);
            e.mem.set_u32(data + 8 * i as u32 + 4, *value);
        }
        data
    }

    fn read(e: &Engine, data: u32, count: usize) -> Vec<(u32, u32)> {
        (0..count as u32)
            .map(|i| (e.mem.u32(data + 8 * i), e.mem.u32(data + 8 * i + 4)))
            .collect()
    }

    #[test]
    fn the_pairs_are_sorted_by_key_and_stay_stable() {
        let mut e = Engine::new();
        let items: Vec<(u32, u32)> = [
            0x0001_0000u32,
            5,
            0xffff_ffff,
            5,
            0x0100_0001,
            0,
            0x8000_0000,
            5,
            0x00ff_ff00,
            1,
            0xdead_beef,
            0x0100_0001,
            2,
            7,
            0x0000_0200,
        ]
        .iter()
        .enumerate()
        .map(|(i, key)| (*key, 100 + i as u32))
        .collect();
        // 16 elements: the count is a multiple of 4.
        let mut items = items;
        items.push((0x1234_5678, 999));
        let data = pairs(&mut e, &items, 0);
        let buffer = e.mem.alloc(8 * items.len() as u32);
        e.call(0x00da_0010, &args![data, items.len() as u32, buffer]);
        let mut want = items.clone();
        want.sort_by_key(|(key, _)| *key);
        assert_eq!(read(&e, data, items.len()), want);
    }

    #[test]
    fn a_count_that_is_not_a_multiple_of_four_moves_the_elements_after_the_end_too() {
        let mut e = Engine::new();
        let items = [(9u32, 1u32), (3, 2), (7, 3), (1, 4), (5, 5)];
        // Three more elements follow in memory; the exe sorts all eight.
        let all = [
            (9u32, 1u32),
            (3, 2),
            (7, 3),
            (1, 4),
            (5, 5),
            (8, 6),
            (2, 7),
            (4, 8),
        ];
        let data = pairs(&mut e, &all, 0);
        let buffer = e.mem.alloc(8 * all.len() as u32);
        e.call(0x00da_0010, &args![data, items.len() as u32, buffer]);
        let mut want = all.to_vec();
        want.sort_by_key(|(key, _)| *key);
        assert_eq!(read(&e, data, 8), want);
    }

    #[test]
    fn a_count_of_zero_or_less_touches_nothing() {
        let mut e = Engine::new();
        let items = [(3u32, 1u32), (2, 2), (1, 3), (0, 4)];
        let data = pairs(&mut e, &items, 0);
        let buffer = e.mem.alloc(32);
        e.mem.set_u32(buffer, 0xabcd);
        e.call(0x00da_0010, &args![data, 0u32, buffer]);
        e.call(0x00da_0010, &args![data, 0xffff_fffeu32, buffer]);
        assert_eq!(read(&e, data, 4), items);
        assert_eq!(e.mem.u32(buffer), 0xabcd);
    }

    #[test]
    fn the_buffer_holds_the_result_of_the_third_pass() {
        let mut e = Engine::new();
        // Keys that differ only in byte 3 are in order after pass 3 only.
        let items = [
            (0x0300_0000u32, 1u32),
            (0x0100_0000, 2),
            (0x0200_0000, 3),
            (0x0000_0000, 4),
        ];
        let data = pairs(&mut e, &items, 0);
        let buffer = e.mem.alloc(32);
        e.call(0x00da_0010, &args![data, 4u32, buffer]);
        assert_eq!(
            read(&e, data, 4),
            [(0, 4), (0x0100_0000, 2), (0x0200_0000, 3), (0x0300_0000, 1)]
        );
        // After pass 2 (the buffer's last write) the order was still the input order.
        assert_eq!(
            read(&e, buffer, 4),
            [(0x0300_0000, 1), (0x0100_0000, 2), (0x0200_0000, 3), (0, 4)]
        );
    }
}
