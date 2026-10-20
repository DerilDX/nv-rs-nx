//! `BSCore/run_00aa4960` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The memory pool for a block size, members of the thread-to-scrap-heap
//! map `BSMap<unsigned long, ScrapHeap*>` (vtable `010a25f8`; its base
//! `BSMapBase` `010a2618`; field and slot names from the Xbox PDB's
//! `BSMapBase`, whose seven slots the PC vtable has in the same places)
//! and `BSCoreMessage`'s constructor and destructor (vtable `010a2708`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `MemoryPool::pMemoryPoolsBySize` (Xbox PDB name, `memorypool.rs`): a
/// pool pointer per 4 bytes of block size.
const POOLS_BY_SIZE: u32 = 0x011f_67b8;
/// The table's entry for 0x240-byte blocks.
const POOL_0X240: u32 = 0x011f_69f8;
/// The `MemoryManager` singleton and its deallocation (`00aa4060`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// The map's vtables (RTTI `.?AV?$BSMap@IPAVScrapHeap@@@@` and
/// `.?AV?$BSMapBase@IPAVScrapHeap@@@@`) and its remove-all (`00ae91f0`).
const SCRAP_HEAP_MAP_VTABLE: u32 = 0x010a_25f8;
const SCRAP_HEAP_MAP_BASE_VTABLE: u32 = 0x010a_2618;
const MAP_REMOVE_ALL: u32 = 0x00ae_91f0;
/// `BSCoreMessage`'s vtable (RTTI `.?AVBSCoreMessage@@`).
const CORE_MESSAGE_VTABLE: u32 = 0x010a_2708;

/// The pool entry for `size` rounded up to `round` (a power of two), or 0
/// above 0x3fc bytes.
fn pool_for(e: &Engine, size: u32, round: u32) -> u32 {
    if size > 0x3fc {
        return 0;
    }
    let rounded = size.wrapping_add(round - 1) & !(round - 1);
    e.mem.u32(POOLS_BY_SIZE + (rounded >> 2) * 4)
}

// Translated from 00aa4960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The memory pool for blocks of `size` bytes (cdecl), or null: below
/// 0x12c the pool of the size rounded up to 16; below 0x200 that pool, or
/// else the one of the size rounded up to 32; below 0x240 the 0x240 pool;
/// none from 0x240 on.
pub fn fn_00aa4960(e: &mut Engine, size: u32) -> u32 {
    let mut pool = 0;
    if size < 0x12c {
        pool = pool_for(e, size, 0x10);
    } else if size < 0x200 {
        pool = pool_for(e, size, 0x10);
        if pool == 0 {
            pool = pool_for(e, size, 0x20);
        }
    } else if size < 0x240 {
        // `CMP 0x3fc, size; SBB; NOT; AND`: the entry, or 0 above 0x3fc.
        let mask = if 0x3fc < size { 0 } else { 0xffff_ffff };
        pool = mask & e.mem.u32(POOL_0X240);
    }
    if pool != 0 {
        pool
    } else {
        0
    }
}

// Translated from 00aa4a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMapBase<unsigned long, ScrapHeap*>::ClearValue` (slot `+0x10`): does
/// nothing to the value.
pub fn fn_00aa4a50(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00aa4a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMap<unsigned long, ScrapHeap*>::~BSMap`: removes every item
/// (`00ae91f0`) under the `BSMap` vtable, then, under the `BSMapBase`
/// vtable, gives the bucket table `m_ppkHashTable` (+8) back to the
/// `MemoryManager` when there is one.
// Translation incomplete: 00aa4a60 - the FS:[0] exception frame (runs the BSMapBase part if the remove-all throws) is not translated
pub fn fn_00aa4a60(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SCRAP_HEAP_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.mem.set_u32(this.addr(), SCRAP_HEAP_MAP_BASE_VTABLE);
    if e.mem.u32(this.addr() + 8) != 0 {
        let table = e.mem.u32(this.addr() + 8);
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, table]);
    }
}

// Translated from 00aa4b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMap<unsigned long, ScrapHeap*>::DeleteItem` (slot `+0x18`): clears
/// the item's `m_val` (+8) and gives the item back to the `MemoryManager`.
pub fn fn_00aa4b20(e: &mut Engine, _this: Ptr, item: Ptr) {
    e.mem.set_u32(item.addr() + 8, 0);
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, item]);
}

// Translated from 00aa4c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCoreMessage::BSCoreMessage` (the class by its vtable): installs the
/// vtable. Returns `this`.
pub fn bs_core_message_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), CORE_MESSAGE_VTABLE);
    this
}

// Translated from 00aa4c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCoreMessage::~BSCoreMessage` (the class by its vtable): restores
/// the vtable.
pub fn bs_core_message_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), CORE_MESSAGE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa4960, fn_00aa4960(u32) -> u32),
        entry!(0x00aa4a50, fn_00aa4a50(Ptr, u32)),
        entry!(0x00aa4a60, fn_00aa4a60(Ptr)),
        entry!(0x00aa4b20, fn_00aa4b20(Ptr, Ptr)),
        entry!(0x00aa4c60, bs_core_message_construct(Ptr) -> Ptr),
        entry!(0x00aa4c80, bs_core_message_destruct(Ptr)),
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

    #[test]
    fn pool_by_size_picks_the_rounded_entry() {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        // Entry i holds 0x1000 + i.
        for i in 0..0x100u32 {
            e.mem.set_u32(POOLS_BY_SIZE + 4 * i, 0x1000 + i);
        }
        let entry = |size: u32| 0x1000 + size / 4;
        // Below 0x12c: rounded to 16.
        assert_eq!(e.call(0x00aa_4960, &args![0x11u32]).u32(), entry(0x20));
        assert_eq!(e.call(0x00aa_4960, &args![0x12bu32]).u32(), entry(0x130));
        // Below 0x200: the 16-rounded entry first...
        assert_eq!(e.call(0x00aa_4960, &args![0x131u32]).u32(), entry(0x140));
        // ...then, when it is empty, the 32-rounded one.
        e.mem.set_u32(POOLS_BY_SIZE + 0x140, 0);
        // (0x131 rounded to 32 is 0x140 too: both empty, null.)
        assert_eq!(e.call(0x00aa_4960, &args![0x131u32]).u32(), 0);
        e.mem.set_u32(POOLS_BY_SIZE + 0x150, 0);
        assert_eq!(e.call(0x00aa_4960, &args![0x141u32]).u32(), entry(0x160));
        // Below 0x240: the 0x240 entry.
        assert_eq!(e.call(0x00aa_4960, &args![0x200u32]).u32(), entry(0x240));
        assert_eq!(e.call(0x00aa_4960, &args![0x23fu32]).u32(), entry(0x240));
        // From 0x240 on: none.
        assert_eq!(e.call(0x00aa_4960, &args![0x240u32]).u32(), 0);
        // An empty entry answers null.
        e.mem.set_u32(POOLS_BY_SIZE + 0x10, 0);
        assert_eq!(e.call(0x00aa_4960, &args![0x8u32]).u32(), 0);
    }

    #[test]
    fn clear_value_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00aa_4a50, &args![0u32, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn map_destructor_removes_all_then_frees_the_table() {
        let mut e = Engine::new();
        e.register(MAP_REMOVE_ALL, |e, a| {
            assert_eq!(e.mem.u32(a[0]), SCRAP_HEAP_MAP_VTABLE);
            Ret::default()
        });
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        for table in [0x4000u32, 0] {
            let map = e.mem.alloc(0x10);
            e.mem.set_u32(map + 8, table);
            e.call_log = Some(vec![]);
            e.call(0x00aa_4a60, &args![map]);
            assert_eq!(e.mem.u32(map), SCRAP_HEAP_MAP_BASE_VTABLE);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, MAP_REMOVE_ALL), vec![vec![map]]);
            let freed = calls(&log, MEMORY_MANAGER_DEALLOCATE);
            if table != 0 {
                assert_eq!(freed, vec![vec![MEMORY_MANAGER, table]]);
            } else {
                assert!(freed.is_empty());
            }
        }
    }

    #[test]
    fn delete_item_clears_the_value_and_frees_it() {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item + 8, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x00aa_4b20, &args![0u32, item]);
        assert_eq!(e.mem.u32(item + 8), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, item]]
        );
    }

    #[test]
    fn core_message_installs_its_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        assert_eq!(e.call(0x00aa_4c60, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), CORE_MESSAGE_VTABLE);
        e.mem.set_u32(this, 0);
        e.call(0x00aa_4c80, &args![this]);
        assert_eq!(e.mem.u32(this), CORE_MESSAGE_VTABLE);
    }
}
