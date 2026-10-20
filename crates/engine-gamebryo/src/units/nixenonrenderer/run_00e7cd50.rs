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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_d7f0, fn_00e7d7f0(Ptr<NiTPointerList>, Ptr) -> u32),
        entry!(0x00e7_ec90, fn_00e7ec90()),
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
