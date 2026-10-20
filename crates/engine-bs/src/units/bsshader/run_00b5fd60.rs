//! `BSShader/run_00b5fd60` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::types::{NiTListItem, NiTPointerList};

/// The list-item pool's lock (`011c5f80`: the `this` of the lock enter `0040fbf0`, `thiscall`
/// with one word, and the lock's owner word), the lock's entry count (`011c5f84`) that the
/// inlined leave decrements, and the head of the pool's free list (`011c5f58`).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;
const LOCK_ENTER: u32 = 0x0040_fbf0;

// Translated from 00b602d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b5fd60`): an `NiTPointerList` remove-head, `thiscall`, no
/// stack words. The head item's next item becomes the head (its previous cleared; the tail
/// cleared instead when there is none); the item's element (+8) is taken and cleared, the item
/// goes back to the item pool inline (the pool's lock entered through `0040fbf0` on `011c5f80`
/// with 0, the item's +4 cleared, the item pushed on the free list at `011c5f58`, the lock
/// left: its count at `011c5f84` down by one and the owner word cleared at zero) and the
/// list's count goes down by one. Answers the element.
pub fn fn_00b602d0(e: &mut Engine, this: Ptr<NiTPointerList>) -> u32 {
    let item: Ptr<NiTListItem> = Ptr::new(e.get(this, NiTPointerList::m_pkHead));
    let next = e.get(item, NiTListItem::m_pkNext);
    e.set(this, NiTPointerList::m_pkHead, next);
    if next != 0 {
        e.mem.set_u32(next.wrapping_add(4), 0);
    } else {
        e.set(this, NiTPointerList::m_pkTail, 0);
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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b6_02d0, fn_00b602d0(Ptr<NiTPointerList>) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A list of two items (elements 10, 20), the pool's lock held by an owner 7.
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
    fn fn_00b602d0_removes_the_head_and_frees_its_item() {
        let (mut e, list, items) = engine();
        assert_eq!(e.call(0x00b6_02d0, &args![list]).u32(), 10);
        assert_eq!(e.mem.u32(list), items[1]);
        assert_eq!(e.mem.u32(items[1] + 4), 0);
        assert_eq!(e.mem.u32(list + 4), items[1]);
        assert_eq!(e.mem.u32(list + 8), 1);
        assert_eq!(e.mem.u32(items[0]), 0x0e10_0000);
        assert_eq!(e.mem.u32(items[0] + 4), 0);
        assert_eq!(e.mem.u32(items[0] + 8), 0);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), items[0]);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 0);
        // The last item: the tail is cleared too.
        assert_eq!(e.call(0x00b6_02d0, &args![list]).u32(), 20);
        assert_eq!(e.mem.u32(list), 0);
        assert_eq!(e.mem.u32(list + 4), 0);
        assert_eq!(e.mem.u32(list + 8), 0);
        assert_eq!(e.mem.u32(items[1]), items[0]);
    }

    #[test]
    fn fn_00b602d0_keeps_a_lock_still_held() {
        let (mut e, list, _) = engine();
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, 1);
        e.call(0x00b6_02d0, &args![list]);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 1);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 7);
    }
}
