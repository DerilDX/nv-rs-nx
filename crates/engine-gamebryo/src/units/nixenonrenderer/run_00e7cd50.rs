//! `NiXenonRenderer/run_00e7cd50` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;

// Translated from 00e7ec90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no unit; no arguments): allocates 0x18 bytes from the memory
/// manager, zeroes the first four words, sets the words at `+0x10` and `+0x14` to 8 and
/// keeps the block in `0126f6cc` (null when the allocation fails).
pub fn fn_00e7ec90(e: &mut Engine) {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x18u32],
        )
        .u32();
    if block == 0 {
        e.mem.set_u32(0x0126_f6cc, 0);
        return;
    }
    e.mem.set_u32(block, 0);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 8);
    e.mem.set_u32(block + 0x14, 8);
    e.mem.set_u32(0x0126_f6cc, block);
}

use crate::types::{NiTListItem, NiTPointerList};

/// The list's remove-head and remove-tail helpers `00b602d0` and `00e7d5b0` (unnamed,
/// `thiscall` on the list, no stack words; they answer the removed element) and the item
/// release `00e6e8a0` (unnamed, `thiscall` on the list, the item).
const REMOVE_HEAD_00B602D0: u32 = 0x00b6_02d0;
const REMOVE_TAIL_00E7D5B0: u32 = 0x00e7_d5b0;
const FREE_ITEM_00E6E8A0: u32 = 0x00e6_e8a0;

// Translated from 00e7d7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`): an `NiTPointerList` remove at a position,
/// `thiscall` with one stack word (the address of the position, an item pointer). At the head
/// the position moves to the next item and `00b602d0` removes the head; at the tail the
/// position becomes null and `00e7d5b0` removes the tail; otherwise the position moves to the
/// next item, the neighbours are linked to each other, the item is released (`00e6e8a0`) and
/// the count at +8 goes down by one. Answers the removed element (what the helpers answer, or
/// the item's element at +8).
pub fn fn_00e7d7f0(e: &mut Engine, this: Ptr<NiTPointerList>, position: Ptr) -> u32 {
    let item: Ptr<NiTListItem> = Ptr::new(e.mem.u32(position.addr()));
    if item.addr() == e.get(this, NiTPointerList::m_pkHead) {
        let next = e.get(item, NiTListItem::m_pkNext);
        e.mem.set_u32(position.addr(), next);
        return e.call(REMOVE_HEAD_00B602D0, &args![this]).u32();
    }
    if item.addr() == e.get(this, NiTPointerList::m_pkTail) {
        e.mem.set_u32(position.addr(), 0);
        return e.call(REMOVE_TAIL_00E7D5B0, &args![this]).u32();
    }
    let previous = e.get(item, NiTListItem::m_pkPrev);
    let next = e.get(item, NiTListItem::m_pkNext);
    e.mem.set_u32(position.addr(), next);
    if previous != 0 {
        e.mem.set_u32(previous, next);
    }
    if next != 0 {
        e.mem.set_u32(next + 4, previous);
    }
    let element = e.get(item, NiTListItem::m_element);
    e.call(FREE_ITEM_00E6E8A0, &args![this, item]);
    let count = e.get(this, NiTPointerList::m_uiCount).wrapping_sub(1);
    e.set(this, NiTPointerList::m_uiCount, count);
    element
}

/// The list-item pool's lock (`011c5f80`: the `this` of the lock enter `0040fbf0`, `thiscall`
/// with one word, and the lock's owner word), the lock's entry count (`011c5f84`) that the
/// inlined leave decrements, and the head of the pool's free list (`011c5f58`).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;
const LOCK_ENTER: u32 = 0x0040_fbf0;

// Translated from 00e7d5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`): an `NiTPointerList` remove-tail, `thiscall`, no
/// stack words. The tail item's previous item becomes the tail (its next cleared; the head
/// cleared instead when there is none); the item's element (+8) is taken and cleared, the item
/// goes back to the item pool inline (the pool's lock entered through `0040fbf0` on `011c5f80`
/// with 0, the item's +4 cleared, the item pushed on the free list at `011c5f58`, the lock
/// left: its count at `011c5f84` down by one and the owner word cleared at zero) and the
/// list's count goes down by one. Answers the element.
pub fn fn_00e7d5b0(e: &mut Engine, this: Ptr<NiTPointerList>) -> u32 {
    let item: Ptr<NiTListItem> = Ptr::new(e.get(this, NiTPointerList::m_pkTail));
    let previous = e.get(item, NiTListItem::m_pkPrev);
    e.set(this, NiTPointerList::m_pkTail, previous);
    if previous != 0 {
        e.mem.set_u32(previous, 0);
    } else {
        e.set(this, NiTPointerList::m_pkHead, 0);
    }
    let element = e.get(item, NiTListItem::m_element);
    e.set(item, NiTListItem::m_element, 0);
    e.call(LOCK_ENTER, &args![NODE_POOL_LOCK, 0u32]);
    e.set(item, NiTListItem::m_pkPrev, 0);
    let head = e.mem.u32(NODE_POOL_FREE_LIST);
    e.set(item, NiTListItem::m_pkNext, head);
    let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
    e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
    e.mem.set_u32(NODE_POOL_FREE_LIST, item.addr());
    if count == 0 {
        e.mem.set_u32(NODE_POOL_LOCK, 0);
    }
    let items = e.get(this, NiTPointerList::m_uiCount).wrapping_sub(1);
    e.set(this, NiTPointerList::m_uiCount, items);
    element
}

/// The empty fixed string (`0109b220`), the string release `00aa04e0` (engine map:
/// `NiShaderFactory::ReleaseGlobalShaderConstant`; `cdecl`, the address of a string word) and
/// `00e89670` (unnamed; `cdecl`, one word).
const FIXED_STRING_EMPTY_0109B220: u32 = 0x0109_b220;
const RELEASE_STRING_00AA04E0: u32 = 0x00aa_04e0;
const RELEASE_00E89670: u32 = 0x00e8_9670;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00e7de30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`): an element's clean-up, `thiscall`, no stack
/// words. When the entry at +0x18 is set, its string at +8 is held (count at string - 8 up
/// through `InterlockedIncrement` unless it is the empty fixed string `0109b220`), copied into
/// a stack word (the one the function saved `ECX` in) handed to `00aa04e0`, and the word's
/// string afterwards let go the same way (`InterlockedDecrement`). Then `00e89670` gets the
/// word at +0x10.
pub fn fn_00e7de30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let entry = e.mem.u32(base + 0x18);
    if entry != 0 {
        let text = e.mem.u32(entry + 8);
        if text != e.global::<u32>(FIXED_STRING_EMPTY_0109B220) {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![text.wrapping_sub(8)]);
        }
        e.with_stack(4, |e, slot| {
            let slot = slot.addr();
            let text = e.mem.u32(entry + 8);
            e.mem.set_u32(slot, text);
            e.call(RELEASE_STRING_00AA04E0, &args![slot]);
            let left = e.mem.u32(slot);
            if left != e.global::<u32>(FIXED_STRING_EMPTY_0109B220) {
                e.call(IMPORT_INTERLOCKED_DECREMENT, &args![left.wrapping_sub(8)]);
            }
        });
    }
    let word = e.mem.u32(base + 0x10);
    e.call(RELEASE_00E89670, &args![word]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_d7f0, fn_00e7d7f0(Ptr<NiTPointerList>, Ptr) -> u32),
        entry!(0x00e7_ec90, fn_00e7ec90()),
        entry!(0x00e7_d5b0, fn_00e7d5b0(Ptr<NiTPointerList>) -> u32),
        entry!(0x00e7_de30, fn_00e7de30(Ptr)),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn builds_the_block_or_stores_null() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let block = e.mem.alloc(0x18);
        e.mem.write(block, &[0xaa; 0x18]);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, a| {
            assert_eq!(a, &[MEMORY_MANAGER_OBJECT, 0x18]);
            block.into_ret()
        });
        e.call(0x00e7_ec90, &args![]);
        assert_eq!(e.mem.u32(0x0126_f6cc), block);
        assert_eq!(e.mem.bytes(block, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.u32(block + 0x10), 8);
        assert_eq!(e.mem.u32(block + 0x14), 8);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00e7_ec90, &args![]);
        assert_eq!(e.mem.u32(0x0126_f6cc), 0);
    }
}

#[cfg(test)]
mod remove_tests {
    use super::*;

    /// A list of three items (elements 10, 20, 30) and a position word.
    fn engine() -> (Engine, u32, [u32; 3], u32) {
        let mut e = Engine::new();
        e.register(REMOVE_HEAD_00B602D0, |_, _| 0x111u32.into_ret());
        e.register(REMOVE_TAIL_00E7D5B0, |_, _| 0x333u32.into_ret());
        e.register(FREE_ITEM_00E6E8A0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let list = e.mem.alloc(0xc);
        let items = [e.mem.alloc(0xc), e.mem.alloc(0xc), e.mem.alloc(0xc)];
        for (i, item) in items.iter().enumerate() {
            let next = if i < 2 { items[i + 1] } else { 0 };
            let previous = if i > 0 { items[i - 1] } else { 0 };
            e.mem.set_u32(*item, next);
            e.mem.set_u32(item + 4, previous);
            e.mem.set_u32(item + 8, 10 * (i as u32 + 1));
        }
        e.mem.set_u32(list, items[0]);
        e.mem.set_u32(list + 4, items[2]);
        e.mem.set_u32(list + 8, 3);
        let position = e.mem.alloc(4);
        (e, list, items, position)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00e7d7f0_middle_item_is_unlinked_and_released() {
        let (mut e, list, items, position) = engine();
        e.mem.set_u32(position, items[1]);
        assert_eq!(e.call(0x00e7_d7f0, &args![list, position]).u32(), 20);
        assert_eq!(e.mem.u32(position), items[2]);
        assert_eq!(e.mem.u32(items[0]), items[2]);
        assert_eq!(e.mem.u32(items[2] + 4), items[0]);
        assert_eq!(e.mem.u32(list + 8), 2);
        assert_eq!(calls(&e, FREE_ITEM_00E6E8A0), vec![vec![list, items[1]]]);
    }

    #[test]
    fn fn_00e7d7f0_head_and_tail_go_to_the_helpers() {
        let (mut e, list, items, position) = engine();
        e.mem.set_u32(position, items[0]);
        assert_eq!(e.call(0x00e7_d7f0, &args![list, position]).u32(), 0x111);
        assert_eq!(e.mem.u32(position), items[1]);
        assert_eq!(calls(&e, REMOVE_HEAD_00B602D0), vec![vec![list]]);
        e.mem.set_u32(position, items[2]);
        assert_eq!(e.call(0x00e7_d7f0, &args![list, position]).u32(), 0x333);
        assert_eq!(e.mem.u32(position), 0);
        assert_eq!(calls(&e, REMOVE_TAIL_00E7D5B0), vec![vec![list]]);
        assert_eq!(e.mem.u32(list + 8), 3);
        assert!(calls(&e, FREE_ITEM_00E6E8A0).is_empty());
    }
}

#[cfg(test)]
mod remove_tail_tests {
    use super::*;

    /// A list of two items (elements 10, 20), the pool's lock taken once by someone else.
    fn engine() -> (Engine, u32, [u32; 2]) {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        e.mem.set_u32(NODE_POOL_FREE_LIST, 0x0e10_0000);
        e.mem.set_u32(NODE_POOL_LOCK, 7);
        e.register(LOCK_ENTER, |e, _| {
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT) + 1;
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let list = e.mem.alloc(0xc);
        let items = [e.mem.alloc(0xc), e.mem.alloc(0xc)];
        e.mem.set_u32(items[0], items[1]);
        e.mem.set_u32(items[0] + 4, 0);
        e.mem.set_u32(items[0] + 8, 10);
        e.mem.set_u32(items[1], 0);
        e.mem.set_u32(items[1] + 4, items[0]);
        e.mem.set_u32(items[1] + 8, 20);
        e.mem.set_u32(list, items[0]);
        e.mem.set_u32(list + 4, items[1]);
        e.mem.set_u32(list + 8, 2);
        (e, list, items)
    }

    #[test]
    fn fn_00e7d5b0_removes_the_tail_and_frees_its_item() {
        let (mut e, list, items) = engine();
        assert_eq!(e.call(0x00e7_d5b0, &args![list]).u32(), 20);
        assert_eq!(e.mem.u32(list + 4), items[0]);
        assert_eq!(e.mem.u32(items[0]), 0);
        assert_eq!(e.mem.u32(list), items[0]);
        assert_eq!(e.mem.u32(list + 8), 1);
        assert_eq!(e.mem.u32(items[1]), 0x0e10_0000);
        assert_eq!(e.mem.u32(items[1] + 4), 0);
        assert_eq!(e.mem.u32(items[1] + 8), 0);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), items[1]);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 0);
        // The last item: the head is cleared too.
        assert_eq!(e.call(0x00e7_d5b0, &args![list]).u32(), 10);
        assert_eq!(e.mem.u32(list), 0);
        assert_eq!(e.mem.u32(list + 4), 0);
        assert_eq!(e.mem.u32(list + 8), 0);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), items[0]);
        assert_eq!(e.mem.u32(items[0]), items[1]);
    }

    #[test]
    fn fn_00e7d5b0_keeps_a_lock_still_held() {
        let (mut e, list, _) = engine();
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, 1);
        e.call(0x00e7_d5b0, &args![list]);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 1);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 7);
    }
}

#[cfg(test)]
mod element_clean_up_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0109_b000, 0x1000);
        e.set_global(FIXED_STRING_EMPTY_0109B220, 0x0e70_0008u32);
        for target in [
            IMPORT_INTERLOCKED_INCREMENT,
            IMPORT_INTERLOCKED_DECREMENT,
            RELEASE_STRING_00AA04E0,
            RELEASE_00E89670,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00e7de30_holds_and_releases_the_entry_string() {
        let mut e = engine();
        let this = e.mem.alloc(0x1c);
        let entry = e.mem.alloc(0xc);
        e.mem.set_u32(entry + 8, 0x0e71_0008);
        e.mem.set_u32(this + 0x18, entry);
        e.mem.set_u32(this + 0x10, 0x55);
        e.call(0x00e7_de30, &args![this]);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_INCREMENT),
            vec![vec![0x0e71_0000]]
        );
        let slot = calls(&e, RELEASE_STRING_00AA04E0)[0][0];
        assert_ne!(slot, entry + 8);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![0x0e71_0000]]
        );
        assert_eq!(calls(&e, RELEASE_00E89670), vec![vec![0x55]]);
    }

    #[test]
    fn fn_00e7de30_skips_the_empty_string_and_a_missing_entry() {
        let mut e = engine();
        let this = e.mem.alloc(0x1c);
        let entry = e.mem.alloc(0xc);
        e.mem.set_u32(entry + 8, 0x0e70_0008);
        e.mem.set_u32(this + 0x18, entry);
        e.call(0x00e7_de30, &args![this]);
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        assert_eq!(calls(&e, RELEASE_STRING_00AA04E0).len(), 1);
        e.mem.set_u32(this + 0x18, 0);
        e.call(0x00e7_de30, &args![this]);
        assert_eq!(calls(&e, RELEASE_STRING_00AA04E0).len(), 1);
        assert_eq!(calls(&e, RELEASE_00E89670).len(), 2);
    }
}
