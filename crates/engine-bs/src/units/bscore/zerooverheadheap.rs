//! `BSCore/zerooverheadheap.cpp` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `ZeroOverheadHeap`: a bump allocator over one block (`pHeap`, `iSize`)
//! whose allocations each carry a one-word size header and are never
//! freed. The layout is the one `run_00aa7030.rs` declares with the
//! class's constructor.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bscore::run_00aa7030::ZeroOverheadHeap;

/// `ZeroOverheadHeap`'s vtable and `IMemoryHeap`'s (RTTI locators).
const ZERO_OVERHEAD_HEAP_VTABLE: u32 = 0x010a_2884;
const I_MEMORY_HEAP_VTABLE: u32 = 0x010a_28c8;
/// The `MemoryManager` singleton, its block free (`00aa42c0`,
/// `thiscall(manager, block, 1)`) and its deallocation (`00aa4060`,
/// `thiscall(manager, block)`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_42c0;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// The spin-lock acquire (`this` = lock, one word: a name string, here
/// null).
const SPIN_LOCK_ENTER: u32 = 0x0040_fbf0;

// Translated from 00aa74c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ZeroOverheadHeap::~ZeroOverheadHeap` (the class by its vtable): gives
/// `pHeap` back to the `MemoryManager` (`00aa42c0(pHeap, 1)`) between
/// restoring the `ZeroOverheadHeap` and the `IMemoryHeap` vtables.
// Translation incomplete: 00aa74c0 - the FS:[0] exception frame (restores the IMemoryHeap vtable if the free throws) is not translated
pub fn zero_overhead_heap_destruct(e: &mut Engine, this: Ptr<ZeroOverheadHeap>) {
    e.set(this, ZeroOverheadHeap::vtable, ZERO_OVERHEAD_HEAP_VTABLE);
    let heap = e.get(this, ZeroOverheadHeap::pHeap);
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, heap, 1u32]);
    e.set(this, ZeroOverheadHeap::vtable, I_MEMORY_HEAP_VTABLE);
}

// Translated from 00aa7530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ZeroOverheadHeap::Allocate` (engine map; two words, the second not
/// read): null without a heap; otherwise, when `pCurrentFree + size + 4`
/// is still below the heap's end, under the spin lock at +0x20 writes the
/// size header at `pCurrentFree`, answers the word after it and moves
/// `pCurrentFree` past the block, counting it in `iAllocations`. A request
/// that does not fit answers null.
pub fn zero_overhead_heap_allocate(
    e: &mut Engine,
    this: Ptr<ZeroOverheadHeap>,
    size: u32,
    _unused_2: u32,
) -> u32 {
    if e.get(this, ZeroOverheadHeap::pHeap) == 0 {
        return 0;
    }
    let mut block = 0;
    let end_of_block = e
        .get(this, ZeroOverheadHeap::pCurrentFree)
        .wrapping_add(size)
        .wrapping_add(4);
    let end_of_heap = e
        .get(this, ZeroOverheadHeap::pHeap)
        .wrapping_add(e.get(this, ZeroOverheadHeap::iSize));
    if end_of_block < end_of_heap {
        let lock = this.addr() + 0x20;
        e.call(SPIN_LOCK_ENTER, &args![lock, 0u32]);
        let header = e.get(this, ZeroOverheadHeap::pCurrentFree);
        e.mem.set_u32(header, size);
        block = e.get(this, ZeroOverheadHeap::pCurrentFree).wrapping_add(4);
        e.set(
            this,
            ZeroOverheadHeap::pCurrentFree,
            block.wrapping_add(size),
        );
        let count = e.get(this, ZeroOverheadHeap::iAllocations).wrapping_add(1);
        e.set(this, ZeroOverheadHeap::iAllocations, count);
        // The inlined spin-lock release.
        let depth = e.get(this, ZeroOverheadHeap::lockCount).wrapping_sub(1);
        e.set(this, ZeroOverheadHeap::lockCount, depth);
        if e.get(this, ZeroOverheadHeap::lockCount) == 0 {
            e.set(this, ZeroOverheadHeap::lockOwner, 0);
        }
    }
    block
}

// Translated from 00aa75e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The heap's free (two words): does nothing (a zero-overhead heap never
/// frees a block).
pub fn fn_00aa75e0(_e: &mut Engine, _this: Ptr<ZeroOverheadHeap>, _unused_1: u32, _unused_2: u32) {}

// Translated from 00aa75f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ZeroOverheadHeap::PointerInHeap` (engine map): whether `pointer` lies
/// strictly after `pHeap` and before `pHeap + iSize`.
pub fn zero_overhead_heap_pointer_in_heap(
    e: &mut Engine,
    this: Ptr<ZeroOverheadHeap>,
    pointer: u32,
) -> bool {
    if pointer <= e.get(this, ZeroOverheadHeap::pHeap) {
        return false;
    }
    let end = e
        .get(this, ZeroOverheadHeap::pHeap)
        .wrapping_add(e.get(this, ZeroOverheadHeap::iSize));
    pointer < end
}

// Translated from 00aa77e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ZeroOverheadHeap`'s scalar deleting destructor (engine map): the
/// destructor, then, with bit 0 of `flags`, the object goes back to the
/// `MemoryManager` (`00aa4060`). Returns `this`.
pub fn zero_overhead_heap_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ZeroOverheadHeap>,
    flags: u32,
) -> Ptr<ZeroOverheadHeap> {
    zero_overhead_heap_destruct(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00aa74c0,
            zero_overhead_heap_destruct(Ptr<ZeroOverheadHeap>)
        ),
        entry!(
            0x00aa7530,
            zero_overhead_heap_allocate(Ptr<ZeroOverheadHeap>, u32, u32) -> u32
        ),
        entry!(0x00aa75e0, fn_00aa75e0(Ptr<ZeroOverheadHeap>, u32, u32)),
        entry!(
            0x00aa75f0,
            zero_overhead_heap_pointer_in_heap(Ptr<ZeroOverheadHeap>, u32) -> bool
        ),
        entry!(
            0x00aa77e0,
            zero_overhead_heap_scalar_deleting_destructor(
                Ptr<ZeroOverheadHeap>,
                u32,
            ) -> Ptr<ZeroOverheadHeap>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn heap(e: &mut Engine, size: u32) -> Ptr<ZeroOverheadHeap> {
        let this: Ptr<ZeroOverheadHeap> = e.new_object();
        let block = e.mem.alloc(size);
        e.set(this, ZeroOverheadHeap::pHeap, block);
        e.set(this, ZeroOverheadHeap::iSize, size);
        e.set(this, ZeroOverheadHeap::pCurrentFree, block);
        this
    }

    #[test]
    fn destructor_frees_the_block_between_the_vtables() {
        let mut e = Engine::new();
        let this = heap(&mut e, 0x40);
        // The free runs with the ZeroOverheadHeap vtable in place.
        let object = this.addr();
        e.register_double(MEMORY_MANAGER_FREE, move |e, _| {
            assert_eq!(e.mem.u32(object), ZERO_OVERHEAD_HEAP_VTABLE);
            Ret::default()
        });
        let block = e.get(this, ZeroOverheadHeap::pHeap);
        e.call_log = Some(vec![]);
        e.call(0x00aa_74c0, &args![this]);
        assert_eq!(e.get(this, ZeroOverheadHeap::vtable), I_MEMORY_HEAP_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, block, 1]]
        );
    }

    #[test]
    fn allocate_bumps_with_a_size_header_under_the_lock() {
        let mut e = Engine::new();
        // The acquire takes the lock once.
        e.register(SPIN_LOCK_ENTER, |e, a| {
            e.mem.set_u32(a[0], 0x77);
            let n = e.mem.u32(a[0] + 4) + 1;
            e.mem.set_u32(a[0] + 4, n);
            Ret::default()
        });
        let this = heap(&mut e, 0x40);
        let base = e.get(this, ZeroOverheadHeap::pHeap);
        e.call_log = Some(vec![]);
        let first = e.call(0x00aa_7530, &args![this, 0x10u32, 0u32]).u32();
        assert_eq!(first, base + 4);
        assert_eq!(e.mem.u32(base), 0x10);
        assert_eq!(e.get(this, ZeroOverheadHeap::pCurrentFree), base + 0x14);
        assert_eq!(e.get(this, ZeroOverheadHeap::iAllocations), 1);
        assert_eq!(e.get(this, ZeroOverheadHeap::lockOwner), 0);
        assert_eq!(e.get(this, ZeroOverheadHeap::lockCount), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, SPIN_LOCK_ENTER),
            vec![vec![this.addr() + 0x20, 0]]
        );
        // 0x14 + 0x27 + 4 = 0x3f fits below 0x40; 0x28 would reach the end.
        assert_eq!(e.call(0x00aa_7530, &args![this, 0x28u32, 0u32]).u32(), 0);
        assert_eq!(
            e.call(0x00aa_7530, &args![this, 0x27u32, 0u32]).u32(),
            base + 0x18
        );
        assert_eq!(e.get(this, ZeroOverheadHeap::iAllocations), 2);
        // Without a heap: null.
        let empty: Ptr<ZeroOverheadHeap> = e.new_object();
        assert_eq!(e.call(0x00aa_7530, &args![empty, 4u32, 0u32]).u32(), 0);
    }

    #[test]
    fn free_does_nothing() {
        let mut e = Engine::new();
        let this = heap(&mut e, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x00aa_75e0, &args![this, 1u32, 2u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn pointer_in_heap_excludes_the_start() {
        let mut e = Engine::new();
        let this = heap(&mut e, 0x40);
        let base = e.get(this, ZeroOverheadHeap::pHeap);
        for (pointer, inside) in [
            (base, false),
            (base + 1, true),
            (base + 0x3f, true),
            (base + 0x40, false),
        ] {
            assert_eq!(e.call(0x00aa_75f0, &args![this, pointer]).bool(), inside);
        }
    }

    #[test]
    fn deleting_destructor_deallocates_on_bit_zero() {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        let this = heap(&mut e, 0x10);
        for (flags, frees) in [(0u32, false), (1, true), (2, false)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00aa_77e0, &args![this, flags]).u32(), this.addr());
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, MEMORY_MANAGER_FREE).len(), 1);
            assert_eq!(
                calls(&log, MEMORY_MANAGER_DEALLOCATE).len(),
                usize::from(frees)
            );
        }
    }
}
