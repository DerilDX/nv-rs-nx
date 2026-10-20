//! `fallout shared/pathfinding/navmeshsearchmaxcost.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006ab480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0106b87c` (RTTI `.?AVNavMeshSearchMaxCost@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `006a6b00`. One word, plain `RET`, returns nothing.
pub fn fn_006ab480(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_b87c);
    e.call(0x006a_6b00, &args![this]);
}

/// Vtable of `BSSimpleArray<NavMeshSearchNode,1024>` (RTTI
/// `.?AV?$BSSimpleArray@UNavMeshSearchNode@@$0EAA@@@`).
const SIMPLE_ARRAY_SEARCH_NODE_VTABLE: u32 = 0x0106_b8a0;
/// Vtable of `BSScrapArray<NavMeshSearchNode,1024>` (RTTI
/// `.?AV?$BSScrapArray@UNavMeshSearchNode@@$0EAA@@@`).
const SCRAP_ARRAY_SEARCH_NODE_VTABLE: u32 = 0x0106_b88c;
/// The array's release (`008454f0`, thiscall, one word: 1 here); unnamed in
/// the engine map.
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// `ScrapHeap::Allocate` (`00aa54a0`, engine map): thiscall on the scrap
/// heap, size and alignment.
const SCRAP_HEAP_ALLOCATE: u32 = 0x00aa_54a0;
/// The scrap-array allocation alignment the code reads (`010a2720`).
const SCRAP_ALIGNMENT: u32 = 0x010a_2720;
/// `BSScrapArray`'s scrap heap pointer (+0x10, as the code reads it).
const SCRAP_HEAP: u32 = 0x10;
/// Size of a `NavMeshSearchNode` (the code's `IMUL ECX,ECX,0x14`).
const SEARCH_NODE_SIZE: u32 = 0x14;

// Translated from 006abc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshSearchNode,1024>`'s destructor body (by the
/// vtable it stores; unnamed in the engine map): the class's vtable, then
/// `008454f0(this, 1)`.
pub fn fn_006abc30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SIMPLE_ARRAY_SEARCH_NODE_VTABLE);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006abc50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006abc50 - the function's C++ exception frame (FS:[0], unwind state) is not translated
/// `BSScrapArray<NavMeshSearchNode,1024>`'s destructor body (by the vtable
/// it stores): the scrap array's vtable, `008454f0(this, 1)`, then the
/// base `BSSimpleArray` destructor body (`006abc30`).
pub fn fn_006abc50(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_SEARCH_NODE_VTABLE);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
    fn_006abc30(e, this);
}

// Translated from 006abcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<NavMeshSearchNode_1024>::_Allocate` (engine map): `count`
/// nodes of 0x14 bytes from the array's scrap heap (+0x10) through
/// `ScrapHeap::Allocate`, aligned as the global at `010a2720` says.
pub fn bs_scrap_array_navmesh_search_node_allocate(e: &mut Engine, this: Ptr, count: u32) -> u32 {
    let alignment: u32 = e.global(SCRAP_ALIGNMENT);
    let heap = e.mem.u32(this.addr() + SCRAP_HEAP);
    e.call(
        SCRAP_HEAP_ALLOCATE,
        &args![heap, count.wrapping_mul(SEARCH_NODE_SIZE), alignment],
    )
    .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006a_b480, fn_006ab480(Ptr)),
        entry!(0x006a_bc30, fn_006abc30(Ptr)),
        entry!(0x006a_bc50, fn_006abc50(Ptr)),
        entry!(
            0x006a_bcb0,
            bs_scrap_array_navmesh_search_node_allocate(Ptr, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_006ab480_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x006a_6b00, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x006a_b480, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x006a_6b00)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0106_b87c);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0106_b87c,
            "the vtable was stored before the call"
        );
    }

    /// Doubles the array release so it records the vtable it sees.
    fn record_release(e: &mut crate::prelude::Engine) {
        use crate::prelude::*;
        e.register(super::ARRAY_RELEASE_008454F0, |e, a| {
            let seen = e.mem.u32(a[0]);
            let count = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 8 + count * 4, seen);
            e.mem.set_u32(a[0] + 4, count + 1);
            Ret::default()
        });
    }

    #[test]
    fn simple_array_destructor_sets_its_vtable_and_releases() {
        use crate::prelude::*;
        let mut e = Engine::new();
        record_release(&mut e);
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x006a_bc30, &args![this]);
        assert_eq!(e.mem.u32(this), super::SIMPLE_ARRAY_SEARCH_NODE_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 1);
        assert_eq!(e.mem.u32(this + 8), super::SIMPLE_ARRAY_SEARCH_NODE_VTABLE);
        let log = e.call_log.take().unwrap();
        let releases: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == super::ARRAY_RELEASE_008454F0)
            .collect();
        assert_eq!(releases.len(), 1);
        assert_eq!(releases[0].1, vec![this, 1]);
    }

    #[test]
    fn scrap_array_destructor_releases_as_scrap_then_as_simple_array() {
        use crate::prelude::*;
        let mut e = Engine::new();
        record_release(&mut e);
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x006a_bc50, &args![this]);
        // Two releases: the first with the scrap array's vtable, the second
        // with the simple array's.
        assert_eq!(e.mem.u32(this + 4), 2);
        assert_eq!(e.mem.u32(this + 8), super::SCRAP_ARRAY_SEARCH_NODE_VTABLE);
        assert_eq!(e.mem.u32(this + 12), super::SIMPLE_ARRAY_SEARCH_NODE_VTABLE);
        assert_eq!(e.mem.u32(this), super::SIMPLE_ARRAY_SEARCH_NODE_VTABLE);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .filter(|(a, _)| *a == super::ARRAY_RELEASE_008454F0)
            .all(|(_, w)| *w == vec![this, 1]));
    }

    #[test]
    fn scrap_array_allocate_asks_its_scrap_heap_for_count_nodes() {
        use crate::prelude::*;
        let mut e = Engine::new();
        e.map(super::SCRAP_ALIGNMENT, 4);
        e.mem.set_u32(super::SCRAP_ALIGNMENT, 4);
        e.register(super::SCRAP_HEAP_ALLOCATE, |_, _| Ret {
            eax: 0x0abc_0000,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + super::SCRAP_HEAP, 0x0555_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a_bcb0, &args![this, 3u32]).u32(), 0x0abc_0000);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == super::SCRAP_HEAP_ALLOCATE)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x0555_0000, 0x3c, 4]);
    }
}
