//! `Havok SDK/run_00d440f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator `00c908c0` grows an array
/// through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;

// Translated from 00d440f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 0x10`: four stack words `first`, `second`, `third`, `fourth`; the
/// names and roles are descriptions, not confirmed): collects, for every key of a shape container,
/// a pair `(key, value)` into the `hkArray` of 8-byte elements at `this + 0xc` (starting with its
/// inline storage at `this + 0x18`, capacity word `0x80000004`).
///
/// Writes `+8` = `fourth`, the vtable word `+0` = `0x010c_f318` and the `u16` at `+6` = 1. With
/// `P = *first` the container is `Q = P.vcall(0x10)`; `n = Q.vcall(4)` is reserved
/// (`max(n, 2 * capacity)` elements of 8 bytes through `00c908c0`, when the capacity is below
/// `n`). Then, for each key from `Q.vcall(8)` (until `0xffffffff`, next from `Q.vcall(0xc)`),
/// the child shape comes from `Q.vcall(0x14, key, buffer)` (a 0x200-byte buffer in the frame) and
/// the object at `*(third + 0xc) + 0xc` is asked through its slot 4 with `(address of a byte,
/// third, second, first, Q, key)`. When the byte it answers a pointer to is not 0 an element is
/// appended: its key, and the result of the function at `table[...]` called with the address of
/// the frame word holding the child shape and `(second, third, fourth)`, where the table row is
/// chosen by the child shape's word at `+0xc` and the byte at `**second + 0xc` (the table is at
/// `*third + 0x5a0`, or `*third + 0x1a0` when the word at `third + 0x14` is 0; a row is 0x20
/// bytes, a function entry 20 bytes at `*third + 0x9a0 + 20 * byte`). Returns `this`.
pub fn fn_00d440f0(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me + 8, fourth);
    e.mem.set_u32(me, 0x010c_f318);
    let array = me + 0xc;
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(array, array + 0xc);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0004);
    e.with_stack(0x240, |e, frame| {
        let t = frame.addr();
        let first_word = e.mem.u32(first + 8);
        e.mem.set_u32(t + 0x28, first_word);
        let holder = e.mem.u32(first);
        e.mem.set_u32(t + 0x2c, first);
        e.mem.set_u32(t + 0x18, me);
        let container = e.vcall(holder, 0x10, &args![]).u32();
        e.mem.set_u32(t + 0xc, container);
        let count = e.vcall(container, 4, &args![]).i32();
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
        if capacity < count {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if count < doubled { doubled } else { count };
            e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 8u32]);
        }
        let container = e.mem.u32(t + 0xc);
        let mut key = e.vcall(container, 8, &args![]).u32();
        if key == 0xffff_ffff {
            return;
        }
        loop {
            let container = e.mem.u32(t + 0xc);
            let child = e.vcall(container, 0x14, &args![key, t + 0x30]).u32();
            let container = e.mem.u32(t + 0xc);
            e.mem.set_u32(t + 0x20, child);
            let owner = e.mem.u32(third + 0xc);
            e.mem.set_u32(t + 0x24, key);
            let embedded = owner + 0xc;
            let vtable = e.mem.u32(embedded);
            let function = e.mem.u32(vtable + 4);
            let answer = e
                .call(
                    function,
                    &args![embedded, t + 0x13, third, second, first, container, key],
                )
                .u32();
            if e.mem.u8(answer) != 0 {
                let size = e.mem.u32(array + 4);
                let data = e.mem.u32(array);
                let element = data.wrapping_add(size.wrapping_mul(8));
                let shape = e.mem.u32(t + 0x20);
                e.mem.set_u32(array + 4, size.wrapping_add(1));
                let selector = e.mem.u32(third + 0x14);
                let rules = e.mem.u32(third);
                e.mem.set_u32(t + 0x14, element);
                let shape_word = e.mem.u32(shape + 0xc);
                let table_row = e.mem.u32(e.mem.u32(second) + 0xc);
                e.mem.set_u32(t + 0x1c, table_row);
                let base = if selector != 0 {
                    rules.wrapping_add(0x5a0)
                } else {
                    rules.wrapping_add(0x1a0)
                };
                let row = (shape_word << 5).wrapping_add(base);
                let index = u32::from(e.mem.u8(row.wrapping_add(table_row)));
                let function = e.mem.u32(rules.wrapping_add(index * 20 + 0x9a0));
                let result = e
                    .call(function, &args![t + 0x20, second, third, fourth])
                    .u32();
                e.mem.set_u32(element + 4, result);
                e.mem.set_u32(element, key);
            }
            let container = e.mem.u32(t + 0xc);
            key = e.vcall(container, 0xc, &args![key]).u32();
            if key == 0xffff_ffff {
                break;
            }
        }
    });
    this
}

// Translated from 00d44310 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `queue`; descriptions, not confirmed): registers two 0x14-byte
/// records with `00cfbbe0` on `queue`: function words `0x00d442c0`, `0x00d34790`, `0x00d347d0`,
/// `0x00d34810` with the bytes `+0x10` = `+0x11` = 1 and the numbers `(0xffffffff, 7)`; then
/// `0x00d44280`, `0x00d43dc0`, `0x00d43960`, `0x00d43b90` with the bytes `(0, 1)` and the numbers
/// `(7, 0xffffffff)`. The padding bytes at `+0x12` are uninitialised in the exe (zero here).
pub fn fn_00d44310(e: &mut Engine, queue: Ptr) {
    e.with_stack(0x14, |e, record| {
        let r = record.addr();
        for (functions, first_byte, first_number, second_number) in [
            (
                [0x00d4_42c0u32, 0x00d3_4790, 0x00d3_47d0, 0x00d3_4810],
                1u8,
                0xffff_ffffu32,
                7u32,
            ),
            (
                [0x00d4_4280, 0x00d4_3dc0, 0x00d4_3960, 0x00d4_3b90],
                0,
                7,
                0xffff_ffff,
            ),
        ] {
            for (index, function) in functions.into_iter().enumerate() {
                e.mem.set_u32(r + index as u32 * 4, function);
            }
            e.mem.set_u8(r + 0x10, first_byte);
            e.mem.set_u8(r + 0x11, 1);
            e.call(
                0x00cf_bbe0,
                &args![queue, record, first_number, second_number],
            );
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_40f0, fn_00d440f0(Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00d4_4310, fn_00d44310(Ptr)),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    struct Scene {
        this: u32,
        first: u32,
        second: u32,
        third: u32,
        container: u32,
        calls: Rc<RefCell<Vec<(u32, Vec<u32>)>>>,
    }

    /// A container with three keys (5, 6, 7) whose children have the shape words 1, 2 and 3; the
    /// embedded checker accepts the keys listed in `accepted`; the rules table maps every row byte
    /// to function entry 1 (a double that answers `0x100 + the shape word`).
    fn scene(e: &mut Engine, accepted: [u32; 3], selector: u32) -> Scene {
        // The container (Q): slots 4 count, 8 first, 0xc next, 0x14 child.
        let container_vtable = e.mem.alloc(0x40);
        for (slot, double) in [
            (4u32, 0x0900_00e1u32),
            (8, 0x0900_00e2),
            (0xc, 0x0900_00e3),
            (0x14, 0x0900_00e4),
        ] {
            e.mem.set_u32(container_vtable + slot, double);
        }
        let container = e.mem.alloc(0x10);
        e.mem.set_u32(container, container_vtable);
        let shapes: Vec<u32> = (1..=3)
            .map(|word| {
                let shape = e.mem.alloc(0x20);
                e.mem.set_u32(shape + 0xc, word);
                shape
            })
            .collect();
        let holder_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(holder_vtable + 0x10, 0x0900_00e5);
        let holder = e.mem.alloc(0x10);
        e.mem.set_u32(holder, holder_vtable);
        let first = e.mem.alloc(0x20);
        e.mem.set_u32(first, holder);
        e.mem.set_u32(first + 8, 0xabcd);
        let calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&calls);
        e.register_double(0x0900_00e5, move |_, a| {
            log.borrow_mut().push((0x0900_00e5, a.to_vec()));
            Ret {
                eax: container,
                ..Ret::default()
            }
        });
        e.register(0x0900_00e1, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.register(0x0900_00e2, |_, _| Ret {
            eax: 5,
            ..Ret::default()
        });
        e.register(0x0900_00e3, |_, a| Ret {
            eax: if a[1] < 7 { a[1] + 1 } else { 0xffff_ffff },
            ..Ret::default()
        });
        let shape_table = shapes.clone();
        e.register_double(0x0900_00e4, move |_, a| Ret {
            eax: shape_table[(a[1] - 5) as usize],
            ..Ret::default()
        });
        // The checker: an embedded object at `*(third + 0xc) + 0xc`; its slot 1 answers a pointer
        // to a byte that is non-zero for the accepted keys.
        let flags = e.mem.alloc(0x10);
        e.mem.set_u8(flags, 0);
        e.mem.set_u8(flags + 1, 1);
        let checker_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(checker_vtable + 4, 0x0900_00e6);
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner + 0xc, checker_vtable);
        let third = e.mem.alloc(0x1000);
        e.mem.set_u32(third + 0xc, owner);
        e.mem.set_u32(third + 0x14, selector);
        let log = Rc::clone(&calls);
        e.register_double(0x0900_00e6, move |_, a| {
            log.borrow_mut().push((0x0900_00e6, a.to_vec()));
            let key = a[6];
            Ret {
                eax: flags + u32::from(accepted.contains(&key)),
                ..Ret::default()
            }
        });
        // `**second + 0xc` is the table column; every row byte selects function entry 1.
        let second_inner = e.mem.alloc(0x20);
        e.mem.set_u32(second_inner + 0xc, 4);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second, second_inner);
        // The rules (`*third`): the table rows, and function entry 1 at `+ 0x9a0 + 20`.
        let rules = e.mem.alloc(0x1000);
        e.mem.set_u32(third, rules);
        for base in [0x5a0u32, 0x1a0] {
            for row in 1..=3u32 {
                e.mem.set_u8(rules + base + (row << 5) + 4, 1);
            }
        }
        e.mem.set_u32(rules + 0x9a0 + 20, 0x0900_00e7);
        let log = Rc::clone(&calls);
        e.register_double(0x0900_00e7, move |e, a| {
            log.borrow_mut().push((0x0900_00e7, a.to_vec()));
            // `a[0]` is the address of the frame word holding the child shape.
            let shape = e.mem.u32(a[0]);
            Ret {
                eax: 0x100 + e.mem.u32(shape + 0xc),
                ..Ret::default()
            }
        });
        e.register_double(MEMORY_ROUTER, |e, _| {
            let router = e.mem.alloc(0x20);
            e.mem.set_u32(router + 0x10, 0x7777);
            Ret {
                eax: router,
                ..Ret::default()
            }
        });
        e.register_double(ARRAY_RESERVE, |e, a| {
            // Gives the array real storage for the wanted number of 8-byte elements.
            let storage = e.mem.alloc(a[2] * 8 + 8);
            e.mem.set_u32(a[1], storage);
            e.mem.set_u32(a[1] + 8, a[2] | 0x8000_0000);
            Ret::default()
        });
        let this = e.mem.alloc(0x60);
        Scene {
            this,
            first,
            second,
            third,
            container,
            calls,
        }
    }

    #[test]
    fn fn_00d440f0_collects_a_pair_for_each_accepted_key() {
        for selector in [0u32, 1] {
            let mut e = Engine::new();
            let s = scene(&mut e, [5, 7, 99], selector);
            let result = e
                .call(
                    0x00d4_40f0,
                    &args![s.this, s.first, s.second, s.third, 0x4321u32],
                )
                .u32();
            assert_eq!(result, s.this);
            assert_eq!(e.mem.u32(s.this + 8), 0x4321);
            assert_eq!(e.mem.u32(s.this), 0x010c_f318);
            assert_eq!(e.mem.u16(s.this + 6), 1);
            // Two elements: key 5 with 0x101 (shape word 1) and key 7 with 0x103.
            let array = s.this + 0xc;
            assert_eq!(e.mem.u32(array + 4), 2);
            let data = e.mem.u32(array);
            assert_eq!(e.mem.u32(data), 5);
            assert_eq!(e.mem.u32(data + 4), 0x101);
            assert_eq!(e.mem.u32(data + 8), 7);
            assert_eq!(e.mem.u32(data + 12), 0x103);
            // The function got `(address of the child shape word, second, third, fourth)`.
            let calls = s.calls.borrow();
            let called: Vec<_> = calls.iter().filter(|(a, _)| *a == 0x0900_00e7).collect();
            assert_eq!(called.len(), 2);
            assert_eq!(called[0].1[1..], [s.second, s.third, 0x4321]);
            // The checker got `(embedded, byte, third, second, first, container, key)`.
            let checked: Vec<_> = calls.iter().filter(|(a, _)| *a == 0x0900_00e6).collect();
            assert_eq!(checked.len(), 3);
            assert_eq!(
                checked[0].1[2..],
                [s.third, s.second, s.first, s.container, 5]
            );
        }
    }

    #[test]
    fn fn_00d440f0_with_no_keys_returns_this_without_checking() {
        let mut e = Engine::new();
        let s = scene(&mut e, [5, 6, 7], 0);
        // A container whose first key is 0xffffffff.
        e.register(0x0900_00e2, |_, _| Ret {
            eax: 0xffff_ffff,
            ..Ret::default()
        });
        e.call(
            0x00d4_40f0,
            &args![s.this, s.first, s.second, s.third, 1u32],
        );
        assert_eq!(e.mem.u32(s.this + 0xc + 4), 0);
        assert!(s.calls.borrow().iter().all(|(a, _)| *a != 0x0900_00e6));
    }

    #[test]
    fn fn_00d440f0_reserves_when_the_count_exceeds_the_capacity() {
        let mut e = Engine::new();
        let s = scene(&mut e, [5, 6, 7], 0);
        e.register(0x0900_00e1, |_, _| Ret {
            eax: 9,
            ..Ret::default()
        });
        e.register(0x0900_00e2, |_, _| Ret {
            eax: 0xffff_ffff,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(
            0x00d4_40f0,
            &args![s.this, s.first, s.second, s.third, 1u32],
        );
        let log = e.call_log.take().unwrap();
        let reserves: Vec<_> = log.iter().filter(|(a, _)| *a == ARRAY_RESERVE).collect();
        // Capacity 4 is below 9: max(9, 2 * 4) elements of 8 bytes.
        assert_eq!(reserves.len(), 1);
        assert_eq!(reserves[0].1, vec![0x7777, s.this + 0xc, 9, 8]);
    }

    #[test]
    fn fn_00d44310_registers_the_two_records() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::<Vec<u32>>::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00cf_bbe0, move |e, a| {
            let mut words = a.to_vec();
            for word in 0..5 {
                words.push(e.mem.u32(a[1] + word * 4));
            }
            log.borrow_mut().push(words);
            Ret::default()
        });
        let queue = e.mem.alloc(0x10);
        e.call(0x00d4_4310, &args![queue]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0][2..4], [0xffff_ffff, 7]);
        assert_eq!(
            seen[0][4..],
            [
                0x00d4_42c0,
                0x00d3_4790,
                0x00d3_47d0,
                0x00d3_4810,
                0x0000_0101
            ]
        );
        assert_eq!(seen[1][2..4], [7, 0xffff_ffff]);
        assert_eq!(
            seen[1][4..],
            [
                0x00d4_4280,
                0x00d4_3dc0,
                0x00d4_3960,
                0x00d4_3b90,
                0x0000_0100
            ]
        );
    }
}
