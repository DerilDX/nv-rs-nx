//! `BSCore/run_00aa7810` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `MemoryHeap` (vtable `010a28fc`, RTTI `.?AVMemoryHeap@@`): its
//! constructor and destructor, and the members its vtable holds at `+0x08`
//! (allocate), `+0x0c` (free), `+0x10` (pointer in heap), `+0x24` (block
//! size), `+0x30` (reserve and commit the heap's memory), `+0x34` (release
//! it), `+0x38` (commit pages) and `+0x3c` (decommit pages); plus an empty
//! `IMemoryHeap` member (slot `+0x2c` of `010a28c8`). The base class's
//! fields are read at their offsets (`AbstractHeap`, Xbox PDB names, as in
//! `run_00aa8ca0.rs`); the two bytes after the base at +0x20f0 and +0x20f1
//! are `MemoryHeap`'s own (not in the Xbox PDB, whose class is larger).

#[allow(unused_imports)]
use crate::prelude::*;

/// `MemoryHeap`'s vtable.
const MEMORY_HEAP_VTABLE: u32 = 0x010a_28fc;
/// `AbstractHeap`'s constructor, heap set-up, tear-down and destructor
/// (`run_00aa8ca0.rs`).
const ABSTRACT_HEAP_CONSTRUCT: u32 = 0x00aa_8ca0;
const ABSTRACT_HEAP_SET_UP: u32 = 0x00aa_8dd0;
const ABSTRACT_HEAP_TEAR_DOWN: u32 = 0x00aa_8ea0;
const ABSTRACT_HEAP_DESTRUCT: u32 = 0x00aa_8da0;
/// `AbstractHeap::BaseAllocate` (decompiler name; `thiscall(heap, size)`).
const BASE_ALLOCATE: u32 = 0x00aa_8f50;
/// The heap's block free (`thiscall(heap, block)`).
const BASE_FREE: u32 = 0x00aa_98f0;
/// The `MemoryManager` singleton and its deallocation
/// (`thiscall(manager, block)`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// Import slots of `VirtualAlloc` and `VirtualFree`.
const VIRTUAL_ALLOC: u32 = 0x00fd_f114;
const VIRTUAL_FREE: u32 = 0x00fd_f10c;
/// `VirtualAlloc`/`VirtualFree` flags.
const MEM_COMMIT: u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;
const MEM_DECOMMIT: u32 = 0x4000;
const MEM_RELEASE: u32 = 0x8000;
const PAGE_READWRITE: u32 = 4;

/// `AbstractHeap` field offsets (Xbox PDB names).
const PAGE_SIZE: u32 = 0x10; // iPageSize
const MEM_HEAP_SIZE: u32 = 0x14; // MemHeapSize
const MEM_HEAP: u32 = 0x30; // pMemHeap
/// `MemoryHeap`'s own bytes (PC): the constructor's last argument, and a
/// flag the destructor sets.
const OWN_FLAG: u32 = 0x20f0;
const DESTROYED: u32 = 0x20f1;

// Translated from 00aa7830 (decompiled, FalloutNV.exe 1.4.0.525)
/// An empty `IMemoryHeap` member (`thiscall`, no arguments; slot `+0x2c` of
/// the `IMemoryHeap` vtable).
pub fn fn_00aa7830(_e: &mut Engine, _this: Ptr) {}

// Translated from 00aa78a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryHeap::MemoryHeap(size, initial_size, name, flag)` (the class by
/// its vtable): the `AbstractHeap` constructor with page size 0x10000, the
/// `MemoryHeap` vtable, `flag` at +0x20f0, 0 at +0x20f1, then the heap
/// set-up `00aa8dd0`. Returns `this`.
// Translation incomplete: 00aa78a0 - the FS:[0] exception frame (runs the AbstractHeap destructor if the set-up throws) is not translated
pub fn memory_heap_construct(
    e: &mut Engine,
    this: Ptr,
    size: u32,
    initial_size: u32,
    name: u32,
    flag: u8,
) -> Ptr {
    e.call(
        ABSTRACT_HEAP_CONSTRUCT,
        &args![this, size, initial_size, name, 0x10000u32],
    );
    e.mem.set_u32(this.addr(), MEMORY_HEAP_VTABLE);
    e.mem.set_u8(this.addr() + OWN_FLAG, flag);
    e.mem.set_u8(this.addr() + DESTROYED, 0);
    e.call(ABSTRACT_HEAP_SET_UP, &args![this]);
    this
}

// Translated from 00aa7930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryHeap::~MemoryHeap` (the class by its vtable): the `MemoryHeap`
/// vtable, the heap tear-down `00aa8ea0`, the byte at +0x20f1 set, then the
/// `AbstractHeap` destructor.
// Translation incomplete: 00aa7930 - the FS:[0] exception frame (runs the AbstractHeap destructor if the tear-down throws) is not translated
pub fn memory_heap_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MEMORY_HEAP_VTABLE);
    e.call(ABSTRACT_HEAP_TEAR_DOWN, &args![this]);
    e.mem.set_u8(this.addr() + DESTROYED, 1);
    e.call(ABSTRACT_HEAP_DESTRUCT, &args![this]);
}

// Translated from 00aa79a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `+0x30`: reserves `reserve` bytes anywhere
/// (`VirtualAlloc(0, reserve, MEM_RESERVE, PAGE_READWRITE)`) and commits
/// the first `commit` bytes of it; when the commit fails the reservation
/// is released and the answer is null. Before that the code reads
/// `iPageSize` (+0x10) and the byte at +0x20f0 into locals it never uses;
/// the reads are kept.
pub fn fn_00aa79a0(e: &mut Engine, this: Ptr, reserve: u32, commit: u32) -> u32 {
    let _page_size = e.mem.u32(this.addr() + PAGE_SIZE);
    let _flag = e.mem.u8(this.addr() + OWN_FLAG);
    let mut memory = e
        .call(
            VIRTUAL_ALLOC,
            &args![0u32, reserve, MEM_RESERVE, PAGE_READWRITE],
        )
        .u32();
    if memory != 0 {
        let committed = e
            .call(
                VIRTUAL_ALLOC,
                &args![memory, commit, MEM_COMMIT, PAGE_READWRITE],
            )
            .u32();
        if committed == 0 {
            e.call(VIRTUAL_FREE, &args![memory, 0u32, MEM_RELEASE]);
            memory = 0;
        }
    }
    memory
}

// Translated from 00aa7a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `+0x34`: releases the heap's memory
/// (`VirtualFree(memory, 0, MEM_RELEASE)`), answering its result.
pub fn fn_00aa7a30(e: &mut Engine, _this: Ptr, memory: u32) -> u32 {
    e.call(VIRTUAL_FREE, &args![memory, 0u32, MEM_RELEASE])
        .u32()
}

// Translated from 00aa7a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `+0x38`: commits `size` bytes at `base + offset`, the size
/// first raised by one page and rounded down to a page
/// (`(size + iPageSize) & !(iPageSize - 1)`); answers that size, or 0 when
/// `VirtualAlloc` fails.
pub fn fn_00aa7a50(e: &mut Engine, this: Ptr, base: u32, offset: u32, size: u32) -> u32 {
    let at = base.wrapping_add(offset);
    let page = e.mem.u32(this.addr() + PAGE_SIZE);
    let mut size = size.wrapping_add(page) & !page.wrapping_sub(1);
    let committed = e
        .call(VIRTUAL_ALLOC, &args![at, size, MEM_COMMIT, PAGE_READWRITE])
        .u32();
    if committed == 0 {
        size = 0;
    }
    size
}

// Translated from 00aa7ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `+0x3c`: rounds `size` down to a page; below one page it
/// answers 0, otherwise decommits that many bytes ending at `base +
/// offset` (`VirtualFree(.., MEM_DECOMMIT)`) and answers the size, or 0
/// when `VirtualFree` does not answer exactly 1.
pub fn fn_00aa7ab0(e: &mut Engine, this: Ptr, base: u32, offset: u32, size: u32) -> u32 {
    let page = e.mem.u32(this.addr() + PAGE_SIZE);
    let mut size = !page.wrapping_sub(1) & size;
    if size < e.mem.u32(this.addr() + PAGE_SIZE) {
        size = 0;
    } else {
        let start = base.wrapping_add(offset).wrapping_sub(size);
        let freed = e
            .call(VIRTUAL_FREE, &args![start, size, MEM_DECOMMIT])
            .u32();
        if freed != 1 {
            size = 0;
        }
    }
    size
}

// Translated from 00aa7b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `+0x08` (allocate; two words, the second not read): the
/// heap's block allocation `00aa8f50(size)`, answering the block.
pub fn fn_00aa7b20(e: &mut Engine, this: Ptr, size: u32, _unused_2: u32) -> u32 {
    e.call(BASE_ALLOCATE, &args![this, size]).u32()
}

// Translated from 00aa7b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `+0x0c` (free; two words, the second not read): frees a
/// non-null block through `00aa98f0`.
pub fn fn_00aa7b40(e: &mut Engine, this: Ptr, block: u32, _unused_2: u32) {
    if block != 0 {
        e.call(BASE_FREE, &args![this, block]);
    }
}

// Translated from 00aa7be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `+0x24` (two words, the second not read): a block's size,
/// the low 28 bits of the header word just before it.
pub fn fn_00aa7be0(e: &mut Engine, _this: Ptr, block: u32, _unused_2: u32) -> u32 {
    e.mem.u32(block.wrapping_sub(4)) & 0x0fff_ffff
}

// Translated from 00aa7e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryHeap`'s scalar deleting destructor (vtable slot `+0`): the
/// destructor, then, with bit 0 of `flags`, the block goes back to the
/// `MemoryManager` singleton (`00aa4060`). Returns `this`.
pub fn fn_00aa7e20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    memory_heap_destruct(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00aa7e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `+0x10`: whether `pointer` lies in `[pMemHeap, pMemHeap +
/// MemHeapSize)`.
pub fn fn_00aa7e70(e: &mut Engine, this: Ptr, pointer: u32) -> bool {
    let start = e.mem.u32(this.addr() + MEM_HEAP);
    if pointer < start {
        return false;
    }
    let end = e
        .mem
        .u32(this.addr() + MEM_HEAP)
        .wrapping_add(e.mem.u32(this.addr() + MEM_HEAP_SIZE));
    pointer < end
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa7830, fn_00aa7830(Ptr)),
        entry!(
            0x00aa78a0,
            memory_heap_construct(Ptr, u32, u32, u32, u8) -> Ptr
        ),
        entry!(0x00aa7930, memory_heap_destruct(Ptr)),
        entry!(0x00aa79a0, fn_00aa79a0(Ptr, u32, u32) -> u32),
        entry!(0x00aa7a30, fn_00aa7a30(Ptr, u32) -> u32),
        entry!(0x00aa7a50, fn_00aa7a50(Ptr, u32, u32, u32) -> u32),
        entry!(0x00aa7ab0, fn_00aa7ab0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00aa7b20, fn_00aa7b20(Ptr, u32, u32) -> u32),
        entry!(0x00aa7b40, fn_00aa7b40(Ptr, u32, u32)),
        entry!(0x00aa7be0, fn_00aa7be0(Ptr, u32, u32) -> u32),
        entry!(0x00aa7e20, fn_00aa7e20(Ptr, u32) -> Ptr),
        entry!(0x00aa7e70, fn_00aa7e70(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_op(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn calls(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn heap(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x20f4);
        e.mem.set_u32(this + PAGE_SIZE, 0x10000);
        this
    }

    #[test]
    fn empty_member_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00aa_7830, &args![0x1000u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn constructor_builds_the_base_then_sets_up() {
        let mut e = Engine::new();
        e.register(ABSTRACT_HEAP_CONSTRUCT, no_op);
        e.register(ABSTRACT_HEAP_SET_UP, |e, a| {
            assert_eq!(e.mem.u32(a[0]), MEMORY_HEAP_VTABLE);
            Ret::default()
        });
        let this = heap(&mut e);
        e.mem.set_u8(this + DESTROYED, 7);
        e.call_log = Some(vec![]);
        let back = e.call(
            0x00aa_78a0,
            &args![this, 0x0c80_0000u32, 0x0550_0000u32, 0x0108_24fcu32, 1u32],
        );
        assert_eq!(back.u32(), this);
        assert_eq!(e.mem.u8(this + OWN_FLAG), 1);
        assert_eq!(e.mem.u8(this + DESTROYED), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, ABSTRACT_HEAP_CONSTRUCT),
            vec![vec![this, 0x0c80_0000, 0x0550_0000, 0x0108_24fc, 0x10000]]
        );
        assert_eq!(calls(&log, ABSTRACT_HEAP_SET_UP), vec![vec![this]]);
    }

    #[test]
    fn destructor_tears_down_then_destroys_the_base() {
        let mut e = Engine::new();
        e.register(ABSTRACT_HEAP_TEAR_DOWN, no_op);
        e.register(ABSTRACT_HEAP_DESTRUCT, |e, a| {
            assert_eq!(e.mem.u8(a[0] + DESTROYED), 1);
            Ret::default()
        });
        let this = heap(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00aa_7930, &args![this]);
        assert_eq!(e.mem.u32(this), MEMORY_HEAP_VTABLE);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|(a, _)| *a).collect();
        assert_eq!(order, vec![ABSTRACT_HEAP_TEAR_DOWN, ABSTRACT_HEAP_DESTRUCT]);
    }

    #[test]
    fn reserve_and_commit_releases_on_a_failed_commit() {
        let mut e = Engine::new();
        let this = heap(&mut e);
        e.register(VIRTUAL_FREE, |_, _| 1u32.into_ret());
        // Both succeed.
        e.register(VIRTUAL_ALLOC, |_, a| {
            (if a[0] == 0 { 0x5000_0000u32 } else { a[0] }).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_79a0, &args![this, 0x1000_0000u32, 0x10_0000u32])
                .u32(),
            0x5000_0000
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, VIRTUAL_ALLOC),
            vec![
                vec![0, 0x1000_0000, MEM_RESERVE, PAGE_READWRITE],
                vec![0x5000_0000, 0x10_0000, MEM_COMMIT, PAGE_READWRITE]
            ]
        );
        assert!(calls(&log, VIRTUAL_FREE).is_empty());
        // The commit fails: released, null.
        e.register(VIRTUAL_ALLOC, |_, a| {
            (if a[0] == 0 { 0x5000_0000u32 } else { 0 }).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_79a0, &args![this, 1u32, 1u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, VIRTUAL_FREE),
            vec![vec![0x5000_0000, 0, MEM_RELEASE]]
        );
        // The reservation fails: null, no commit.
        e.register(VIRTUAL_ALLOC, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_79a0, &args![this, 1u32, 1u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, VIRTUAL_ALLOC).len(), 1);
    }

    #[test]
    fn release_frees_the_memory() {
        let mut e = Engine::new();
        e.register(VIRTUAL_FREE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_7a30, &args![0u32, 0x5000_0000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, VIRTUAL_FREE),
            vec![vec![0x5000_0000, 0, MEM_RELEASE]]
        );
    }

    #[test]
    fn commit_raises_by_a_page_and_rounds() {
        let mut e = Engine::new();
        let this = heap(&mut e);
        e.register(VIRTUAL_ALLOC, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let size = e.call(
            0x00aa_7a50,
            &args![this, 0x5000_0000u32, 0x2_0000u32, 0x1234u32],
        );
        assert_eq!(size.u32(), 0x1_0000);
        // An exact page still gains one.
        let size = e.call(0x00aa_7a50, &args![this, 0x5000_0000u32, 0u32, 0x1_0000u32]);
        assert_eq!(size.u32(), 0x2_0000);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, VIRTUAL_ALLOC)[0],
            vec![0x5002_0000, 0x1_0000, MEM_COMMIT, PAGE_READWRITE]
        );
        e.register(VIRTUAL_ALLOC, |_, _| 0u32.into_ret());
        let size = e.call(0x00aa_7a50, &args![this, 0x5000_0000u32, 0u32, 0x10u32]);
        assert_eq!(size.u32(), 0);
    }

    #[test]
    fn decommit_rounds_down_and_needs_a_whole_page() {
        let mut e = Engine::new();
        let this = heap(&mut e);
        e.register(VIRTUAL_FREE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        let size = e.call(
            0x00aa_7ab0,
            &args![this, 0x5000_0000u32, 0x4_0000u32, 0x2_1234u32],
        );
        assert_eq!(size.u32(), 0x2_0000);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, VIRTUAL_FREE),
            vec![vec![0x5002_0000, 0x2_0000, MEM_DECOMMIT]]
        );
        e.call_log = Some(vec![]);
        let size = e.call(
            0x00aa_7ab0,
            &args![this, 0x5000_0000u32, 0x4_0000u32, 0xffffu32],
        );
        assert_eq!(size.u32(), 0);
        assert!(calls(&e.call_log.take().unwrap(), VIRTUAL_FREE).is_empty());
        // VirtualFree answering anything but 1 gives 0.
        e.register(VIRTUAL_FREE, |_, _| 2u32.into_ret());
        let size = e.call(
            0x00aa_7ab0,
            &args![this, 0x5000_0000u32, 0x4_0000u32, 0x1_0000u32],
        );
        assert_eq!(size.u32(), 0);
    }

    #[test]
    fn allocate_and_free_forward_to_the_base() {
        let mut e = Engine::new();
        e.register(BASE_ALLOCATE, |_, a| (a[1] + 0x100).into_ret());
        e.register(BASE_FREE, no_op);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_7b20, &args![0x10u32, 0x20u32, 9u32]).u32(),
            0x120
        );
        e.call(0x00aa_7b40, &args![0x10u32, 0u32, 9u32]);
        e.call(0x00aa_7b40, &args![0x10u32, 0x3000u32, 9u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, BASE_ALLOCATE), vec![vec![0x10, 0x20]]);
        assert_eq!(calls(&log, BASE_FREE), vec![vec![0x10, 0x3000]]);
    }

    #[test]
    fn block_size_is_the_header_low_28_bits() {
        let mut e = Engine::new();
        let block = e.mem.alloc(8);
        e.mem.set_u32(block, 0xf000_0123);
        assert_eq!(
            e.call(0x00aa_7be0, &args![0u32, block + 4, 0u32]).u32(),
            0x123
        );
    }

    #[test]
    fn deleting_destructor_returns_the_block_on_bit_zero() {
        let mut e = Engine::new();
        e.register(ABSTRACT_HEAP_TEAR_DOWN, no_op);
        e.register(ABSTRACT_HEAP_DESTRUCT, no_op);
        e.register(MEMORY_MANAGER_DEALLOCATE, no_op);
        let this = heap(&mut e);
        for (flags, frees) in [(0u32, false), (1, true), (2, false)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00aa_7e20, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, ABSTRACT_HEAP_TEAR_DOWN), vec![vec![this]]);
            let freed = calls(&log, MEMORY_MANAGER_DEALLOCATE);
            if frees {
                assert_eq!(freed, vec![vec![MEMORY_MANAGER, this]]);
            } else {
                assert!(freed.is_empty());
            }
        }
    }

    #[test]
    fn pointer_in_heap_checks_the_range() {
        let mut e = Engine::new();
        let this = heap(&mut e);
        e.mem.set_u32(this + MEM_HEAP, 0x5000_0000);
        e.mem.set_u32(this + MEM_HEAP_SIZE, 0x1000);
        for (pointer, inside) in [
            (0x4fff_ffffu32, false),
            (0x5000_0000, true),
            (0x5000_0fff, true),
            (0x5000_1000, false),
        ] {
            assert_eq!(e.call(0x00aa_7e70, &args![this, pointer]).bool(), inside);
        }
    }
}
