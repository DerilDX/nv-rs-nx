//! `Havok SDK/hkfreelistallocator.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The Xbox PDB has no entry for `hkFreeListAllocator`, so its layout is described
//! by offsets, each named where a function uses it: `+0` virtual table, `+4` the
//! critical section, `+0x24` the parent allocator, `+0x28` an
//! `hkLargeBlockAllocator`, `+0xec` a table of 0x29 free-list pointers indexed by
//! size / 16, `+0x190` the list of distinct free lists and `+0x234` their count,
//! `+0x238` the next unused element of the pool of 0x29 free lists (0x34 bytes
//! each) that starts at `+0x240` and ends at `+0x23c`, `+0xa94` the soft limit.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkFreeListAllocator`'s virtual table (`010d1464`; the class name is confirmed
/// by the RTTI record before it).
const VTABLE: u32 = 0x010d_1464;
/// `EnterCriticalSection` and `LeaveCriticalSection` (`kernel32` import slots,
/// `stdcall`, one word).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `hkCriticalSection::hkCriticalSection` (`00cf6da0`, `thiscall`, spin count),
/// `hkLargeBlockAllocator::hkLargeBlockAllocator` (`00dd8d40`, `thiscall`, parent),
/// the free list's default constructor (`00dd7110`), constructor wrapper
/// (`00dd7650`: element size, alignment, block size, allocator) and clear
/// (`00dd7200`).
const CRITICAL_SECTION_CONSTRUCTOR: u32 = 0x00cf_6da0;
const LARGE_BLOCK_ALLOCATOR_CONSTRUCTOR: u32 = 0x00dd_8d40;
const FREE_LIST_DEFAULT_CONSTRUCTOR: u32 = 0x00dd_7110;
const FREE_LIST_CONSTRUCTOR: u32 = 0x00dd_7650;
const FREE_LIST_CLEAR: u32 = 0x00dd_7200;

const FREE_LIST_SIZE: u32 = 0x34;
const FREE_LIST_COUNT: u32 = 0x29;
const LARGE_BLOCK_ALLOCATOR: u32 = 0x28;
const SIZE_TABLE: u32 = 0xec;
const LIST_OF_LISTS: u32 = 0x190;
const LIST_COUNT: u32 = 0x234;
const POOL_NEXT: u32 = 0x238;
const POOL_END: u32 = 0x23c;
const POOL: u32 = 0x240;

// Translated from 00d5fb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::setMemorySoftLimit` (Xbox PDB), `thiscall`, `RET 4`: stores
/// `limit` at `+0xa94` between `EnterCriticalSection` and `LeaveCriticalSection` of
/// the section at `+4`.
pub fn hk_free_list_allocator_set_memory_soft_limit(e: &mut Engine, this: Ptr, limit: u32) {
    let section = this.addr() + 4;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    e.mem.set_u32(this.addr() + 0xa94, limit);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00d5fc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): gives a free list back to the
/// pool. Clears it (`00dd7200`) and, when it is the last element handed out (it
/// ends at `+0x238`), takes that element back by moving `+0x238` down 0x34.
pub fn fn_00d5fc70(e: &mut Engine, this: Ptr, free_list: u32) {
    e.call(FREE_LIST_CLEAR, &args![free_list]);
    let next = e.mem.u32(this.addr() + POOL_NEXT);
    if free_list.wrapping_add(FREE_LIST_SIZE) == next {
        e.mem
            .set_u32(this.addr() + POOL_NEXT, next.wrapping_sub(FREE_LIST_SIZE));
    }
}

// Translated from 00d60020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: element size, alignment, block
/// size): the free list for blocks of that size. Takes the next free list of the
/// pool (`+0x238`, 0x34 bytes on) and constructs it (`00dd7650`, with the large
/// block allocator at `+0x28`). When a list in the list of lists (`+0x190`, `+0x234`
/// entries) has the same rounded element size (the word at `+4`) the new one is
/// given back (`00d5fc70`) and that list is returned; otherwise the new list is
/// appended and returned. The exe executes `INT3` when the pool is used up (`+0x238`
/// not below `+0x23c`): that is a panic here.
pub fn fn_00d60020(
    e: &mut Engine,
    this: Ptr,
    element_size: u32,
    alignment: u32,
    block_size: u32,
) -> u32 {
    let base = this.addr();
    let next = e.mem.u32(base + POOL_NEXT);
    if next >= e.mem.u32(base + POOL_END) {
        panic!("FalloutNV.exe executes INT3 at 00d60034: the pool of free lists is used up");
    }
    let new_list = next;
    e.mem
        .set_u32(base + POOL_NEXT, new_list.wrapping_add(FREE_LIST_SIZE));
    if new_list != 0 {
        e.call(
            FREE_LIST_CONSTRUCTOR,
            &args![
                new_list,
                element_size,
                alignment,
                block_size,
                base + LARGE_BLOCK_ALLOCATOR
            ],
        );
    }
    let count = e.mem.u32(base + LIST_COUNT) as i32;
    let rounded = e.mem.u32(new_list + 4);
    let mut index = 0u32;
    while (index as i32) < count {
        let list = e.mem.u32(base + LIST_OF_LISTS + index * 4);
        if e.mem.u32(list + 4) == rounded {
            e.call(0x00d5_fc70, &args![base, new_list]);
            return e.mem.u32(base + LIST_OF_LISTS + index * 4);
        }
        index += 1;
    }
    e.mem.set_u32(
        base + LIST_OF_LISTS + (count as u32).wrapping_mul(4),
        new_list,
    );
    let stored = e.mem.u32(base + LIST_COUNT);
    e.mem.set_u32(base + LIST_COUNT, stored.wrapping_add(1));
    new_list
}

// Translated from 00d60270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::hkFreeListAllocator` (Xbox PDB; the map gives the address
/// no name), `thiscall`, `RET 4`: stores the virtual table, constructs the critical
/// section at `+4` (spin count 0xfa0), stores `parent` at `+0x24` and constructs
/// the large block allocator at `+0x28` with it. Constructs the pool of 0x29
/// default free lists at `+0x240`, clears `+0x234`, `+0xa98`, `+0x1c` and `+0x20`,
/// sets the soft limit to `0x7fffffff` (`00d5fb70`) and the pool bounds. Clears the
/// 0x29-entry table at `+0xec` and gives it the free lists for the sizes 0x200
/// (`+0x16c`), 0x100 (`+0x12c`), 0 (`+0xec`) and 0x220 (`+0x210`..`+0x280`,
/// table indexes 0x21 to 0x28), then, from table index 0x28 down to 1, one list
/// (`00d60020`) for every slot still empty: 16 / 32 byte alignment (32 from index
/// 4 up) and a block size of `index * 16` times 16, or 0x1000 below 0x100.
/// Returns `this`.
pub fn hk_free_list_allocator_hk_free_list_allocator(
    e: &mut Engine,
    this: Ptr,
    parent: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    e.call(CRITICAL_SECTION_CONSTRUCTOR, &args![base + 4, 0xfa0u32]);
    e.mem.set_u32(base + 0x24, parent);
    e.call(
        LARGE_BLOCK_ALLOCATOR_CONSTRUCTOR,
        &args![base + LARGE_BLOCK_ALLOCATOR, parent],
    );
    e.mem.set_u32(base + LIST_COUNT, 0);
    for index in 0..=0x28u32 {
        e.call(
            FREE_LIST_DEFAULT_CONSTRUCTOR,
            &args![base + POOL + index * FREE_LIST_SIZE],
        );
    }
    e.mem.set_u32(base + 0xa98, 0);
    e.mem.set_u32(base + 0x1c, 0);
    e.mem.set_u32(base + 0x20, 0);
    hk_free_list_allocator_set_memory_soft_limit(e, this, 0x7fff_ffff);
    e.mem.set_u32(base + POOL_NEXT, base + POOL);
    e.mem.set_u32(
        base + POOL_END,
        base + POOL + FREE_LIST_COUNT * FREE_LIST_SIZE,
    );
    for index in 0..FREE_LIST_COUNT {
        e.mem.set_u32(base + SIZE_TABLE + index * 4, 0);
    }
    let list = fn_00d60020(e, this, 0x200, 0x80, 0x1_0000);
    e.mem.set_u32(base + 0x16c, list);
    let list = fn_00d60020(e, this, 0x100, 0x20, 0x8000);
    e.mem.set_u32(base + 0x12c, list);
    let list = fn_00d60020(e, this, 0, 4, 0x100);
    e.mem.set_u32(base + SIZE_TABLE, list);
    let list = fn_00d60020(e, this, 0x220, 0x80, 0x5000);
    for size in 0x210..=0x280u32 {
        e.mem.set_u32(base + SIZE_TABLE + (size >> 4) * 4, list);
    }
    for size in (0x10..=0x280u32).rev().step_by(0x10) {
        let slot = base + SIZE_TABLE + (size >> 4) * 4;
        if e.mem.u32(slot) == 0 {
            let alignment = if size < 0x40 { 0x10 } else { 0x20 };
            let block_size = if size >= 0x100 { size << 4 } else { 0x1000 };
            let list = fn_00d60020(e, this, size, alignment, block_size);
            e.mem.set_u32(slot, list);
        }
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d5fb70,
            hk_free_list_allocator_set_memory_soft_limit(Ptr, u32)
        ),
        entry!(0x00d5fc70, fn_00d5fc70(Ptr, u32)),
        entry!(0x00d60020, fn_00d60020(Ptr, u32, u32, u32) -> u32),
        entry!(
            0x00d60270,
            hk_free_list_allocator_hk_free_list_allocator(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIZE: u32 = 0xaa0;

    fn calls_to(log: &[(u32, Vec<u32>)], target: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == target)
            .map(|call| call.1.clone())
            .collect()
    }

    fn allocator_object(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(SIZE + 0x20);
        e.mem.write(object, &vec![0xcd; SIZE as usize]);
        object
    }

    /// The free list constructor double stores the rounded size at +4, as
    /// `element size` rounded to 16.
    fn free_list_double(e: &mut Engine) {
        e.register(FREE_LIST_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 4, (a[1] + 15) & !15);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(FREE_LIST_CLEAR, |_, _| Ret::default());
    }

    #[test]
    fn the_soft_limit_is_stored_inside_the_critical_section() {
        let mut e = Engine::new();
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        let object = allocator_object(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00d5_fb70, &args![object, 0x1234u32]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|call| call.0).collect();
        assert_eq!(
            order,
            vec![0x00d5_fb70, ENTER_CRITICAL_SECTION, LEAVE_CRITICAL_SECTION]
        );
        assert_eq!(log[1].1, vec![object + 4]);
        assert_eq!(log[2].1, vec![object + 4]);
        assert_eq!(e.mem.u32(object + 0xa94), 0x1234);
    }

    #[test]
    fn giving_back_the_last_list_moves_the_pool_down() {
        let mut e = Engine::new();
        free_list_double(&mut e);
        let object = allocator_object(&mut e);
        let last = object + 0x240 + 0x34;
        e.mem.set_u32(object + 0x238, last + 0x34);
        e.call_log = Some(vec![]);
        e.call(0x00d5_fc70, &args![object, last]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FREE_LIST_CLEAR), vec![vec![last]]);
        assert_eq!(e.mem.u32(object + 0x238), last);
        // A list that is not the last one leaves the pool alone.
        e.call(0x00d5_fc70, &args![object, object + 0x300]);
        assert_eq!(e.mem.u32(object + 0x238), last);
    }

    #[test]
    fn a_new_size_appends_a_list_and_a_known_size_gives_it_back() {
        let mut e = Engine::new();
        free_list_double(&mut e);
        let object = allocator_object(&mut e);
        e.mem.set_u32(object + 0x234, 0);
        e.mem.set_u32(object + 0x238, object + 0x240);
        e.mem.set_u32(object + 0x23c, object + 0x240 + 0x34 * 4);
        e.call_log = Some(vec![]);
        let first = e.call(0x00d6_0020, &args![object, 0x40u32, 0x10u32, 0x800u32]);
        assert_eq!(first.u32(), object + 0x240);
        assert_eq!(e.mem.u32(object + 0x234), 1);
        assert_eq!(e.mem.u32(object + 0x190), object + 0x240);
        assert_eq!(e.mem.u32(object + 0x238), object + 0x240 + 0x34);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FREE_LIST_CONSTRUCTOR),
            vec![vec![object + 0x240, 0x40, 0x10, 0x800, object + 0x28]]
        );
        // Another size gets its own list.
        let second = e.call(0x00d6_0020, &args![object, 0x80u32, 0x10u32, 0x800u32]);
        assert_eq!(second.u32(), object + 0x240 + 0x34);
        assert_eq!(e.mem.u32(object + 0x234), 2);
        // The same rounded size (0x35 rounds to 0x40 in the double) returns the
        // first list; the pool element is given back.
        e.call_log = Some(vec![]);
        let again = e.call(0x00d6_0020, &args![object, 0x35u32, 0x10u32, 0x800u32]);
        assert_eq!(again.u32(), object + 0x240);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FREE_LIST_CLEAR),
            vec![vec![object + 0x240 + 0x34 * 2]]
        );
        assert_eq!(e.mem.u32(object + 0x234), 2);
        assert_eq!(e.mem.u32(object + 0x238), object + 0x240 + 0x34 * 2);
    }

    #[test]
    #[should_panic(expected = "INT3 at 00d60034")]
    fn an_exhausted_pool_stops_where_the_exe_executes_int3() {
        let mut e = Engine::new();
        free_list_double(&mut e);
        let object = allocator_object(&mut e);
        e.mem.set_u32(object + 0x238, object + 0x240);
        e.mem.set_u32(object + 0x23c, object + 0x240);
        e.call(0x00d6_0020, &args![object, 0x40u32, 0x10u32, 0x800u32]);
    }

    fn constructor_engine() -> Engine {
        let mut e = Engine::new();
        e.register(CRITICAL_SECTION_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(LARGE_BLOCK_ALLOCATOR_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(FREE_LIST_DEFAULT_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        // Every list is distinct: rounded size = element size + the list's
        // address, so no pool element is given back.
        e.register(FREE_LIST_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 4, a[0]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(FREE_LIST_CLEAR, |_, _| Ret::default());
        e
    }

    #[test]
    fn the_constructor_builds_the_pool_and_the_size_table() {
        let mut e = constructor_engine();
        let object = allocator_object(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d6_0270, &args![object, 0x7000u32]).u32(), object);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(
            calls_to(&log, CRITICAL_SECTION_CONSTRUCTOR),
            vec![vec![object + 4, 0xfa0]]
        );
        assert_eq!(
            calls_to(&log, LARGE_BLOCK_ALLOCATOR_CONSTRUCTOR),
            vec![vec![object + 0x28, 0x7000]]
        );
        assert_eq!(e.mem.u32(object + 0x24), 0x7000);
        assert_eq!(calls_to(&log, FREE_LIST_DEFAULT_CONSTRUCTOR).len(), 41);
        assert_eq!(e.mem.u32(object + 0xa94), 0x7fff_ffff);
        assert_eq!(e.mem.u32(object + 0xa98), 0);
        assert_eq!(e.mem.u32(object + 0x1c), 0);
        assert_eq!(e.mem.u32(object + 0x20), 0);
        assert_eq!(e.mem.u32(object + 0x23c), object + 0x240 + 0x854);
        // The first four lists, in the order of the calls.
        let made = calls_to(&log, FREE_LIST_CONSTRUCTOR);
        let pool = |n: u32| object + 0x240 + n * 0x34;
        assert_eq!(made[0], vec![pool(0), 0x200, 0x80, 0x1_0000, object + 0x28]);
        assert_eq!(made[1], vec![pool(1), 0x100, 0x20, 0x8000, object + 0x28]);
        assert_eq!(made[2], vec![pool(2), 0, 4, 0x100, object + 0x28]);
        assert_eq!(made[3], vec![pool(3), 0x220, 0x80, 0x5000, object + 0x28]);
        assert_eq!(e.mem.u32(object + 0x16c), pool(0));
        assert_eq!(e.mem.u32(object + 0x12c), pool(1));
        assert_eq!(e.mem.u32(object + 0xec), pool(2));
        // Sizes 0x210 to 0x280 share the fourth list: table indexes 0x21 to 0x28.
        for index in 0x21..=0x28 {
            assert_eq!(e.mem.u32(object + 0xec + index * 4), pool(3));
        }
        // The remaining slots (1 to 0x20, except 0x12c/0x16c = 0x10, 0x16 ... see
        // below) each got a list, from 0x20 down to 1.
        let rest = &made[4..];
        // Table indexes 0x20 down to 1 minus the two already set (0x12c >> 4 is
        // out of table range; 0x12c and 0x16c are +0x40 and +0x80 from +0xec,
        // that is indexes 0x10 and 0x20).
        assert_eq!(rest.len(), 0x20 - 2);
        // First of the rest is index 0x1f (size 0x1f0): alignment 0x20, block
        // size 0x1f00.
        assert_eq!(rest[0][1..], [0x1f0, 0x20, 0x1f00, object + 0x28]);
        // The last is index 1 (size 0x10): alignment 0x10, block size 0x1000.
        assert_eq!(
            rest[rest.len() - 1][1..],
            [0x10, 0x10, 0x1000, object + 0x28]
        );
        // The pool holds 4 + 30 lists in all.
        assert_eq!(e.mem.u32(object + 0x234), 34);
        assert_eq!(e.mem.u32(object + 0x238), pool(34));
    }
}
