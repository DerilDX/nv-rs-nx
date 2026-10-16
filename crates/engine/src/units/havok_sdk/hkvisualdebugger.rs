//! `Havok SDK/hkvisualdebugger.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00cd1e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cd_1970`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cd1e40(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cd_1970, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`, cdecl `(allocator, array, count,
/// elementSize)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkMemUtil::memMove` (Xbox PDB, `00d2cc70`, cdecl `(destination, source, bytes)`).
const MEM_MOVE: u32 = 0x00d2_cc70;

// Translated from 00cd1d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Splices into an `hkArray` of 4-byte elements (`data`, `size`, `capacityAndFlags` at +0,
/// +4, +8): removes `remove_count` elements at `index` and inserts `insert_count` words
/// copied from `source` there. When the new size exceeds the capacity (low 30 bits) the
/// array is grown to the larger of the new size and twice the capacity with `00c908c0`
/// (`allocator` as its first argument, element size 4); the tail is moved with `memMove` and
/// the words are copied one at a time. The class is unconfirmed (the template instance
/// sits in this unit).
pub fn fn_00cd1d80(
    e: &mut Engine,
    this: Ptr,
    allocator: u32,
    index: i32,
    remove_count: i32,
    source: u32,
    insert_count: i32,
) {
    let size = e.mem.i32(this.addr() + 4);
    let tail_count = size.wrapping_sub(index).wrapping_sub(remove_count);
    let new_size = size.wrapping_sub(remove_count).wrapping_add(insert_count);
    let capacity = (e.mem.u32(this.addr() + 8) & 0x3fff_ffff) as i32;
    if capacity < new_size {
        let doubled = capacity.wrapping_add(capacity);
        let count = if new_size < doubled {
            doubled
        } else {
            new_size
        };
        e.call(ARRAY_RESERVE, &args![allocator, this, count, 4u32]);
    }
    let data = e.mem.u32(this.addr());
    let source_start = data.wrapping_add((index.wrapping_add(remove_count) as u32).wrapping_mul(4));
    let destination = data.wrapping_add((index.wrapping_add(insert_count) as u32).wrapping_mul(4));
    e.call(
        MEM_MOVE,
        &args![
            destination,
            source_start,
            (tail_count as u32).wrapping_mul(4)
        ],
    );
    let mut target = e
        .mem
        .u32(this.addr())
        .wrapping_add((index as u32).wrapping_mul(4));
    let mut from = source;
    let mut remaining = insert_count;
    while remaining > 0 {
        let word = e.mem.u32(from);
        e.mem.set_u32(target, word);
        target = target.wrapping_add(4);
        from = from.wrapping_add(4);
        remaining -= 1;
    }
    e.mem.set_i32(this.addr() + 4, new_size);
}

// Translated from 00cd1e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts `count` words from `source` at `index` of the array: `00cd1d80` with nothing
/// removed. The class is unconfirmed.
pub fn fn_00cd1e20(e: &mut Engine, this: Ptr, allocator: u32, index: i32, source: u32, count: i32) {
    fn_00cd1d80(e, this, allocator, index, 0, source, count);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd_1e40, deleting_destructor_00cd1e40(Ptr, u8) -> Ptr),
        entry!(0x00cd_1d80, fn_00cd1d80(Ptr, u32, i32, i32, u32, i32)),
        entry!(0x00cd_1e20, fn_00cd1e20(Ptr, u32, i32, u32, i32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00cd1e40_frees_only_when_bit_zero_is_set() {
        check(0x00cd_1e40, 0x00cd_1970);
    }
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

    type Reserved = Rc<RefCell<Vec<Vec<u32>>>>;

    /// An array of the given words with room for `capacity`, `_reserve` that moves it to a
    /// fresh block of the requested count, and a real `memMove`.
    fn rig(words: &[u32], capacity: u32) -> (Engine, u32, Reserved) {
        let mut e = test_engine();
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(capacity * 4);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *w);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, words.len() as u32);
        e.mem.set_u32(array + 8, 0x8000_0000 | capacity);
        let reserved: Reserved = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&reserved);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            let old = e.mem.u32(a[1]);
            let size = e.mem.u32(a[1] + 4);
            let block = e.mem.alloc(a[2] * 4);
            let bytes = e.mem.bytes(old, size * 4);
            e.mem.write(block, &bytes);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        e.register_double(MEM_MOVE, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        (e, array, reserved)
    }

    fn contents(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    #[test]
    fn splice_replaces_a_range_without_growing() {
        let (mut e, array, reserved) = rig(&[1, 2, 3, 4, 5], 8);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 9);
        e.call(
            0x00cd_1d80,
            &args![array, 0x55u32, 1u32, 2u32, source, 1u32],
        );
        assert_eq!(contents(&e, array), vec![1, 9, 4, 5]);
        assert!(reserved.borrow().is_empty());
    }

    #[test]
    fn insert_grows_to_the_larger_of_the_new_size_and_twice_the_capacity() {
        let (mut e, array, reserved) = rig(&[1, 2, 3, 4], 4);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 9);
        e.mem.set_u32(source + 4, 8);
        e.call(0x00cd_1e20, &args![array, 0x55u32, 1u32, source, 2u32]);
        assert_eq!(contents(&e, array), vec![1, 9, 8, 2, 3, 4]);
        // Twice the capacity (8) beats the new size (6).
        assert_eq!(*reserved.borrow(), vec![vec![0x55, array, 8, 4]]);
        let (mut e, array, reserved) = rig(&[1], 1);
        let source = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_u32(source + 4 * i, 10 + i);
        }
        e.call(0x00cd_1e20, &args![array, 7u32, 1u32, source, 4u32]);
        assert_eq!(contents(&e, array), vec![1, 10, 11, 12, 13]);
        // The new size (5) beats twice the capacity (2).
        assert_eq!(*reserved.borrow(), vec![vec![7, array, 5, 4]]);
    }

    #[test]
    fn a_non_positive_count_copies_nothing() {
        let (mut e, array, _) = rig(&[1, 2, 3], 4);
        let source = e.mem.alloc(8);
        e.call(0x00cd_1e20, &args![array, 0u32, 1u32, source, 0u32]);
        assert_eq!(contents(&e, array), vec![1, 2, 3]);
        e.call(0x00cd_1d80, &args![array, 0u32, 0u32, 3u32, source, 0u32]);
        assert_eq!(contents(&e, array), Vec::<u32>::new());
    }
}
