//! `fallout/ai/combat/combatplanner.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00995870` calls on the member (`00aa6330`).
const CALLEE_00AA6330: u32 = 0x00aa_6330;

/// The function `fn_00995890` calls on the member (`00aa62e0`).
const CALLEE_00AA62E0: u32 = 0x00aa_62e0;

// Translated from 00995870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00aa6330` with `ECX` = `this + 0x14` and passes on its `bool` result (`AL`).
pub fn fn_00995870(e: &mut Engine, this: Ptr) -> bool {
    e.call(CALLEE_00AA6330, &args![this.byte_add(0x14)]).bool()
}

// Translated from 00995890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00aa62e0` with `ECX` = `this + 0x14` and passes on nothing (the callee returns no value).
pub fn fn_00995890(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_00AA62E0, &args![this.byte_add(0x14)]);
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVCombatPlanNode@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108DC80: u32 = 0x0108_dc80;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVCombatPlanLink@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108DCA8: u32 = 0x0108_dca8;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAV?$AStarNode@PAVCombatPlanNode@@PAVCombatPlanLink@@@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108DCBC: u32 = 0x0108_dcbc;

// Translated from 00996540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVCombatPlanNode@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_00996540(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108DC80);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 00996640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVCombatPlanLink@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_00996640(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108DCA8);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 009966f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAV?$AStarNode@PAVCombatPlanNode@@PAVCombatPlanLink@@@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_009966f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108DCBC);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// ---- Batch b012 --------------------------------------------------------------------------

/// `00719b20` (`thiscall`, `RET 0xc`; engine-map unit `interfacemanager.cpp`): a search
/// answering an index or -1, given the key, 0 and the compare function `009a3830`.
const FIND_INDEX: u32 = 0x0071_9b20;
const COMPARE_009A3830: u32 = 0x009a_3830;
/// `ScrapHeap::Allocate` (`00aa54a0`, `thiscall`, `RET 8`: size, alignment) and
/// `ScrapHeap::Deallocate` (`00aa5610`, `thiscall`, `RET 4`), on the scrap heap a
/// `BSScrapArray` keeps at `+0x10`; the alignment word they pass (`010a2720`).
const SCRAP_ALLOCATE: u32 = 0x00aa_54a0;
const SCRAP_DEALLOCATE: u32 = 0x00aa_5610;
const SCRAP_ALIGNMENT: u32 = 0x010a_2720;
/// `0044ddc0` (`thiscall`, no stack word): the array's element count; `00401460` (`cdecl`,
/// `(to, from, bytes)`): the copy `_Reallocate` makes.
const ARRAY_COUNT: u32 = 0x0044_ddc0;
const COPY_BYTES: u32 = 0x0040_1460;
/// `00403d30` (`cdecl`, `(block, value, bytes)`; Ghidra names it `_Fiopen`): fills a block.
const FILL_BYTES: u32 = 0x0040_3d30;
/// `BSSimpleArray<TESObjectWEAP *,1024>::SetSize` (engine map, `006f2290`, `thiscall`,
/// `RET 8`) and `00877a30` (`thiscall`, `RET 4`: the address of an element).
const ARRAY_SET_SIZE: u32 = 0x006f_2290;
const ARRAY_ELEMENT: u32 = 0x0087_7a30;

// Translated from 009962f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether `00719b20(key, 0, 009a3830)` on
/// `this` finds an index (anything but -1).
pub fn fn_009962f0(e: &mut Engine, this: Ptr, key: u32) -> bool {
    e.call(FIND_INDEX, &args![this, key, 0u32, COMPARE_009A3830])
        .u32()
        != 0xffff_ffff
}

// Translated from 006f2870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<CombatPlanNode *,1024>::_Allocate` (Xbox PDB name from the engine map;
/// `thiscall`, `RET 4`): `count * 4` bytes from the array's scrap heap (`+0x10`) with the
/// alignment word at `010a2720`.
pub fn bs_scrap_array_combat_plan_node_allocate(e: &mut Engine, this: Ptr, count: u32) -> u32 {
    let alignment = e.mem.u32(SCRAP_ALIGNMENT);
    let heap = e.mem.u32(this.addr() + 0x10);
    e.call(SCRAP_ALLOCATE, &args![heap, count << 2, alignment])
        .u32()
}

// Translated from 006cf4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<CombatPlanNode *,1024>::_Deallocate` (Xbox PDB name from the engine map;
/// `thiscall`, `RET 4`): gives `block` back to the array's scrap heap (`+0x10`).
pub fn bs_scrap_array_combat_plan_node_deallocate(e: &mut Engine, this: Ptr, block: u32) {
    let heap = e.mem.u32(this.addr() + 0x10);
    e.call(SCRAP_DEALLOCATE, &args![heap, block]);
}

// Translated from 006cfc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<CombatPlanNode *,1024>::_Reallocate` (Xbox PDB name from the engine map;
/// `thiscall`, `RET 8`): a block of `count * 4` bytes from the scrap heap, the array's
/// elements (`0044ddc0` of them, 4 bytes each) copied from `old` (`00401460`), `old` given
/// back, and the new block answered.
pub fn bs_scrap_array_combat_plan_node_reallocate(
    e: &mut Engine,
    this: Ptr,
    old: u32,
    count: u32,
) -> u32 {
    let alignment = e.mem.u32(SCRAP_ALIGNMENT);
    let heap = e.mem.u32(this.addr() + 0x10);
    let block = e
        .call(SCRAP_ALLOCATE, &args![heap, count << 2, alignment])
        .u32();
    let elements = e.call(ARRAY_COUNT, &args![this]).u32();
    e.call(COPY_BYTES, &args![block, old, elements << 2]);
    let heap = e.mem.u32(this.addr() + 0x10);
    e.call(SCRAP_DEALLOCATE, &args![heap, old]);
    block
}

// Translated from 00996840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`, `this` not read): a hash, `key % 0x7f7`.
pub fn fn_00996840(_e: &mut Engine, _this: Ptr, key: u32) -> u32 {
    key % 0x7f7
}

// Translated from 00996800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): when the count word at `+0x1fdc` is not 0,
/// clears the 0x1fdc bytes before it (`00403d30(this, 0, 0x1fdc)`) and the count.
pub fn fn_00996800(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr() + 0x1fdc) != 0 {
        e.call(FILL_BYTES, &args![this, 0u32, 0x1fdcu32]);
        e.mem.set_u32(this.addr() + 0x1fdc, 0);
    }
}

// Translated from 00996ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`, `this` not read): turns the chain of
/// path nodes from `node` (each linked to the next at `+0x10`) into two arrays, back to
/// front: `nodes` sized to the chain length and `links` to one less (`006f2290` with 1);
/// walking the chain with `i` from the last index down, `nodes[i]` gets the node's `+8`
/// and, while `i > 0`, `links[i - 1]` its `+0xc` (each element through `00877a30`). False
/// for a null `node`, true otherwise.
pub fn fn_00996ae0(e: &mut Engine, _this: Ptr, node: u32, nodes: Ptr, links: Ptr) -> bool {
    if node == 0 {
        return false;
    }
    let mut count = 0u32;
    let mut walk = node;
    while walk != 0 {
        count = count.wrapping_add(1);
        walk = e.mem.u32(walk + 0x10);
    }
    e.call(ARRAY_SET_SIZE, &args![nodes, count, 1u32]);
    e.call(ARRAY_SET_SIZE, &args![links, count.wrapping_sub(1), 1u32]);
    let mut walk = node;
    let mut index = count.wrapping_sub(1) as i32;
    while walk != 0 {
        let slot = e.call(ARRAY_ELEMENT, &args![nodes, index as u32]).u32();
        let value = e.mem.u32(walk + 8);
        e.mem.set_u32(slot, value);
        if index > 0 {
            let slot = e
                .call(ARRAY_ELEMENT, &args![links, index.wrapping_sub(1) as u32])
                .u32();
            let value = e.mem.u32(walk + 0xc);
            e.mem.set_u32(slot, value);
        }
        walk = e.mem.u32(walk + 0x10);
        index = index.wrapping_sub(1);
    }
    true
}

// Translated from 00996ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): takes the first entry of the first non-empty of
/// the 20 buckets at `this + 4`: the bucket gets the entry's next (`+0x18`; the next's back
/// link `+0x14` set to the bucket's address) or 0, the entry's links are cleared, and the
/// entry answered; 0 when every bucket is empty.
pub fn fn_00996ce0(e: &mut Engine, this: Ptr) -> u32 {
    for index in 0..0x14u32 {
        let bucket = this.addr() + 4 + 4 * index;
        let entry = e.mem.u32(bucket);
        if entry == 0 {
            continue;
        }
        let next = e.mem.u32(entry + 0x18);
        if next == 0 {
            e.mem.set_u32(bucket, 0);
        } else {
            e.mem.set_u32(next + 0x14, bucket);
            let next = e.mem.u32(e.mem.u32(bucket) + 0x18);
            e.mem.set_u32(bucket, next);
        }
        e.mem.set_u32(entry + 0x14, 0);
        e.mem.set_u32(entry + 0x18, 0);
        return entry;
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0099_5870, fn_00995870(Ptr) -> bool),
        entry!(0x0099_5890, fn_00995890(Ptr)),
        entry!(0x0099_6540, fn_00996540(Ptr)),
        entry!(0x0099_6640, fn_00996640(Ptr)),
        entry!(0x0099_66f0, fn_009966f0(Ptr)),
        entry!(0x0099_62f0, fn_009962f0(Ptr, u32) -> bool),
        entry!(0x006f_2870, bs_scrap_array_combat_plan_node_allocate(Ptr, u32) -> u32),
        entry!(
            0x006c_f4a0,
            bs_scrap_array_combat_plan_node_deallocate(Ptr, u32)
        ),
        entry!(
            0x006c_fc20,
            bs_scrap_array_combat_plan_node_reallocate(Ptr, u32, u32) -> u32
        ),
        entry!(0x0099_6840, fn_00996840(Ptr, u32) -> u32),
        entry!(0x0099_6800, fn_00996800(Ptr)),
        entry!(0x0099_6ae0, fn_00996ae0(Ptr, u32, Ptr, Ptr) -> bool),
        entry!(0x0099_6ce0, fn_00996ce0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00995870_passes_this_plus_14_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00AA6330, |_, a| Ret {
            eax: a[0] & 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0099_5870, &args![0x4100u32]).bool());
        assert!(e.call(0x0099_5870, &args![0x4100u32 + 1]).bool());
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00AA6330).collect();
        assert_eq!(inner.len(), 2);
        assert_eq!(inner[0].1, vec![0x4100 + 0x14]);
    }

    #[test]
    fn l68d_00995890_passes_this_plus_14_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00AA62E0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0099_5890, &args![0x4200u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00AA62E0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4200 + 0x14]);
    }
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_00996540_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0099_6540, ARRAY_VTABLE_0108DC80, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_00996640_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0099_6640, ARRAY_VTABLE_0108DCA8, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_009966f0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0099_66f0, ARRAY_VTABLE_0108DCBC, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod b012_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn finding_answers_whether_an_index_came_back() {
        let mut e = Engine::new();
        e.register(FIND_INDEX, |_, a| {
            if a[1] == 7 { 3u32 } else { 0xffff_ffff }.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0099_62f0, &args![0x5000u32, 7u32]).bool());
        assert!(!e.call(0x0099_62f0, &args![0x5000u32, 8u32]).bool());
        assert_eq!(
            calls(&e, FIND_INDEX)[0],
            vec![0x5000, 7, 0, COMPARE_009A3830]
        );
    }

    #[test]
    fn the_scrap_array_allocator() {
        let mut e = Engine::new();
        e.map(0x010a_2000, 0x1000);
        e.mem.set_u32(SCRAP_ALIGNMENT, 4);
        e.register(SCRAP_ALLOCATE, |_, a| (0x7000 + a[1]).into_ret());
        e.register(SCRAP_DEALLOCATE, |_, _| Ret::default());
        e.register(ARRAY_COUNT, |_, _| 3u32.into_ret());
        e.register(COPY_BYTES, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x10, 0x6600);
        assert_eq!(e.call(0x006f_2870, &args![this, 5u32]).u32(), 0x7014);
        e.call(0x006c_f4a0, &args![this, 0x9000u32]);
        assert_eq!(
            e.call(0x006c_fc20, &args![this, 0x9100u32, 8u32]).u32(),
            0x7020
        );
        assert_eq!(
            calls(&e, SCRAP_ALLOCATE),
            vec![vec![0x6600, 20, 4], vec![0x6600, 32, 4]]
        );
        assert_eq!(calls(&e, COPY_BYTES), vec![vec![0x7020, 0x9100, 12]]);
        assert_eq!(
            calls(&e, SCRAP_DEALLOCATE),
            vec![vec![0x6600, 0x9000], vec![0x6600, 0x9100]]
        );
    }

    #[test]
    fn the_hash_and_the_clear() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0099_6840, &args![0u32, 0x7f8u32]).u32(), 1);
        e.register(FILL_BYTES, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x1fe0);
        e.call(0x0099_6800, &args![this]);
        assert!(calls(&e, FILL_BYTES).is_empty());
        e.mem.set_u32(this + 0x1fdc, 4);
        e.call(0x0099_6800, &args![this]);
        assert_eq!(calls(&e, FILL_BYTES), vec![vec![this, 0, 0x1fdc]]);
        assert_eq!(e.mem.u32(this + 0x1fdc), 0);
    }

    #[test]
    fn a_node_chain_becomes_two_arrays_back_to_front() {
        let mut e = Engine::new();
        let nodes_array = e.mem.alloc(0x40);
        let links_array = e.mem.alloc(0x40);
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(ARRAY_ELEMENT, |_, a| (a[0] + 4 * a[1]).into_ret());
        e.call_log = Some(vec![]);
        let chain: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x14)).collect();
        for (index, node) in chain.iter().enumerate() {
            e.mem.set_u32(node + 8, 0x10 + index as u32);
            e.mem.set_u32(node + 0xc, 0x20 + index as u32);
            let next = chain.get(index + 1).copied().unwrap_or(0);
            e.mem.set_u32(node + 0x10, next);
        }
        let answer = e.call(
            0x0099_6ae0,
            &args![0u32, chain[0], nodes_array, links_array],
        );
        assert!(answer.bool());
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE),
            vec![vec![nodes_array, 3, 1], vec![links_array, 2, 1]]
        );
        let nodes: Vec<u32> = (0..3).map(|i| e.mem.u32(nodes_array + 4 * i)).collect();
        let links: Vec<u32> = (0..2).map(|i| e.mem.u32(links_array + 4 * i)).collect();
        assert_eq!(nodes, vec![0x12, 0x11, 0x10]);
        assert_eq!(links, vec![0x21, 0x20]);
        assert!(!e
            .call(0x0099_6ae0, &args![0u32, 0u32, nodes_array, links_array])
            .bool());
    }

    #[test]
    fn the_first_bucket_entry_is_taken() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x54);
        assert_eq!(e.call(0x0099_6ce0, &args![this]).u32(), 0);
        let (first, second) = (e.mem.alloc(0x1c), e.mem.alloc(0x1c));
        e.mem.set_u32(first + 0x18, second);
        e.mem.set_u32(first + 0x14, 0x77);
        e.mem.set_u32(this + 4 + 4 * 3, first);
        assert_eq!(e.call(0x0099_6ce0, &args![this]).u32(), first);
        assert_eq!(e.mem.u32(this + 0x10), second);
        assert_eq!(e.mem.u32(second + 0x14), this + 0x10);
        assert_eq!((e.mem.u32(first + 0x14), e.mem.u32(first + 0x18)), (0, 0));
        assert_eq!(e.call(0x0099_6ce0, &args![this]).u32(), second);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }
}
