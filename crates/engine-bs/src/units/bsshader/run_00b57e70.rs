//! `BSShader/run_00b57e70` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::types::{NiTListItem, NiTPointerList};

/// The list-item allocation `0043a010` (`thiscall` on the list's allocator at +8, no stack
/// words; answers the new item).
const LIST_NEW_ITEM_0043A010: u32 = 0x0043_a010;

// Translated from 00b58570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b57e70`); `thiscall` on an `NiTPointerList` with one stack
/// word, the address of the element (a reference): adds the element at the tail. A new item
/// from the allocator at +8 (`0043a010`) gets the element, no next item and the old tail as
/// its previous one; the old tail (if any) links to it, the count at +8 goes up by one and
/// the item becomes the tail (and also the head when the list was empty).
pub fn fn_00b58570(e: &mut Engine, this: Ptr<NiTPointerList>, element: Ptr) {
    let item: Ptr<NiTListItem> = e
        .call(LIST_NEW_ITEM_0043A010, &args![this.addr() + 8])
        .ptr();
    let value = e.mem.u32(element.addr());
    e.set(item, NiTListItem::m_element, value);
    e.set(item, NiTListItem::m_pkNext, 0);
    let tail = e.get(this, NiTPointerList::m_pkTail);
    e.set(item, NiTListItem::m_pkPrev, tail);
    let count = e.get(this, NiTPointerList::m_uiCount).wrapping_add(1);
    if tail != 0 {
        e.mem.set_u32(tail, item.addr());
        e.set(this, NiTPointerList::m_uiCount, count);
    } else {
        e.set(this, NiTPointerList::m_uiCount, count);
        e.set(this, NiTPointerList::m_pkHead, item.addr());
    }
    e.set(this, NiTPointerList::m_pkTail, item.addr());
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b5_8570, fn_00b58570(Ptr<NiTPointerList>, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(LIST_NEW_ITEM_0043A010, |e, _| e.mem.alloc(0xc).into_ret());
        e
    }

    #[test]
    fn fn_00b58570_adds_the_first_item_as_head_and_tail() {
        let mut e = engine();
        let list = e.mem.alloc(0xc);
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, 0xabcd);
        e.call(0x00b5_8570, &args![list, element]);
        let item = e.mem.u32(list);
        assert_ne!(item, 0);
        assert_eq!(e.mem.u32(list + 4), item);
        assert_eq!(e.mem.u32(list + 8), 1);
        assert_eq!(e.mem.u32(item), 0);
        assert_eq!(e.mem.u32(item + 4), 0);
        assert_eq!(e.mem.u32(item + 8), 0xabcd);
    }

    #[test]
    fn fn_00b58570_links_after_the_tail() {
        let mut e = engine();
        let list = e.mem.alloc(0xc);
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, 1);
        e.call(0x00b5_8570, &args![list, element]);
        let first = e.mem.u32(list);
        e.mem.set_u32(element, 2);
        e.call(0x00b5_8570, &args![list, element]);
        let second = e.mem.u32(list + 4);
        assert_eq!(e.mem.u32(list), first);
        assert_ne!(second, first);
        assert_eq!(e.mem.u32(first), second);
        assert_eq!(e.mem.u32(second + 4), first);
        assert_eq!(e.mem.u32(second + 8), 2);
        assert_eq!(e.mem.u32(list + 8), 2);
    }
}
