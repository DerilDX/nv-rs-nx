//! `fallout shared/testopic.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Finding topics: a topic of a kind by index (`TESTopic::GetTopic`), a
//! topic info by form id in a topic's per-quest info lists (`00619e10`),
//! and the topic that holds an info (`TESTopic::GetOwnerTopic`).

#[allow(unused_imports)]
use crate::prelude::*;

/// The topic counts per kind (`0119a41c`, one signed word per kind).
const TOPIC_COUNTS: u32 = 0x0119_a41c;
/// The topic arrays per kind (`0119a43c`, one word per kind: the address of
/// 0xc bytes per topic, whose first word `GetTopic` returns).
const TOPIC_ARRAYS: u32 = 0x0119_a43c;
const TOPIC_ENTRY_SIZE: u32 = 0xc;
/// The word at `011c3f2c`: the object whose list (`004612e0`) holds the
/// topics.
const TOPIC_LIST_OWNER: u32 = 0x011c_3f2c;
/// `004612e0` (`ECX` = object): `this + 0x108` (the topic list's first
/// node).
const LIST_AT_0X108: u32 = 0x0046_12e0;
/// `005d8a70` (`ECX` = topic): `this + 0x2c` (the list of the topic's
/// per-quest info lists).
const TOPIC_QUEST_LIST: u32 = 0x005d_8a70;
/// `006815c0` (`ECX` = node): the node itself (its first word is the item).
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`ECX` = node): the word at +4 (the next node).
const NODE_NEXT: u32 = 0x0072_6070;
/// `00717e50` (`ECX` = object): `this + 4` (in a per-quest entry, its info
/// array).
const PLUS_FOUR: u32 = 0x0071_7e50;
/// `0084e3a0` (`ECX` = object): the word at +0xc (an array's count; a
/// form's id).
const WORD_AT_0XC: u32 = 0x0084_e3a0;
/// `00619f20` (cdecl: per-quest entry, index): the info at that index.
const ENTRY_INFO: u32 = 0x0061_9f20;

// Translated from 0061a2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTopic::GetTopic` (Xbox PDB; cdecl: kind, index): the first word of
/// entry `index` of the kind's topic array (`0119a43c`), or 0 for a
/// negative index or one not below the kind's count (`0119a41c`; both
/// compared signed).
pub fn tes_topic_get_topic(e: &mut Engine, kind: u32, index: i32) -> Ptr {
    if index < 0 {
        return Ptr::NULL;
    }
    let count = e.mem.i32(TOPIC_COUNTS.wrapping_add(kind.wrapping_mul(4)));
    if index >= count {
        return Ptr::NULL;
    }
    let array = e.mem.u32(TOPIC_ARRAYS.wrapping_add(kind.wrapping_mul(4)));
    Ptr::new(
        e.mem.u32(
            (index as u32)
                .wrapping_mul(TOPIC_ENTRY_SIZE)
                .wrapping_add(array),
        ),
    )
}

// Translated from 00619e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The topic info with form id `id` in the topic's (`ECX`) per-quest info
/// lists (two stack words, the second a byte; `RET 8`). The list
/// (`005d8a70`) is walked node by node until a node with no item; for each
/// entry with a nonzero info count (`0084e3a0` of `00717e50`), the infos
/// (`00619f20`) are searched from the last down to index 0 (signed) when
/// `last_first` is nonzero, else from the first up (unsigned); the first
/// non-null info whose id (`0084e3a0`) is `id` is returned. 0 if none. The
/// engine map gives it no name.
pub fn fn_00619e10(e: &mut Engine, this: Ptr, id: u32, last_first: u8) -> Ptr {
    let mut node = e.call(TOPIC_QUEST_LIST, &args![this]).u32();
    loop {
        if node == 0 {
            return Ptr::NULL;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            return Ptr::NULL;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(item);
        node = e.call(NODE_NEXT, &args![node]).u32();
        let infos = e.call(PLUS_FOUR, &args![entry]).u32();
        let count = e.call(WORD_AT_0XC, &args![infos]).u32();
        if count == 0 {
            continue;
        }
        if last_first != 0 {
            let mut index = count.wrapping_sub(1) as i32;
            while index >= 0 {
                let info = e.call(ENTRY_INFO, &args![entry, index]).u32();
                if info != 0 && e.call(WORD_AT_0XC, &args![info]).u32() == id {
                    return Ptr::new(info);
                }
                index -= 1;
            }
        } else {
            let mut index = 0u32;
            while index < count {
                let info = e.call(ENTRY_INFO, &args![entry, index]).u32();
                if info != 0 && e.call(WORD_AT_0XC, &args![info]).u32() == id {
                    return Ptr::new(info);
                }
                index += 1;
            }
        }
    }
}

// Translated from 0061a310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTopic::GetOwnerTopic` (Xbox PDB; cdecl: the topic info): walks the
/// topic list (`004612e0` of the object at `011c3f2c`) node by node until a
/// node with no item, and returns the first topic whose info lists hold an
/// info with this info's id (`0084e3a0`; `00619e10`, first to last) that is
/// this very info. 0 if none.
pub fn tes_topic_get_owner_topic(e: &mut Engine, info: Ptr) -> Ptr {
    let owner = e.mem.u32(TOPIC_LIST_OWNER);
    let mut node = e.call(LIST_AT_0X108, &args![owner]).u32();
    loop {
        if node == 0 {
            return Ptr::NULL;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            return Ptr::NULL;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let topic = e.mem.u32(item);
        node = e.call(NODE_NEXT, &args![node]).u32();
        let id = e.call(WORD_AT_0XC, &args![info]).u32();
        if fn_00619e10(e, Ptr::new(topic), id, 0).addr() == info.addr() {
            return Ptr::new(topic);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_9e10, fn_00619e10(Ptr, u32, u8) -> Ptr),
        entry!(0x0061_a2d0, tes_topic_get_topic(u32, i32) -> Ptr),
        entry!(0x0061_a310, tes_topic_get_owner_topic(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// The doubles behave as the exe's bodies: a node's item is its first
    /// word, the next node its second; an entry's infos are at +4, with the
    /// count at +0xc of that and the infos in an array at +0x10 of the
    /// entry; a form's id is at +0xc.
    fn setup() -> Engine {
        let mut e = Engine::new();
        e.register(TOPIC_QUEST_LIST, |_, a| ret(a[0] + 0x2c));
        e.register(LIST_AT_0X108, |_, a| ret(a[0] + 0x108));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(PLUS_FOUR, |_, a| ret(a[0] + 4));
        e.register(WORD_AT_0XC, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(ENTRY_INFO, |e, a| {
            let array = e.mem.u32(a[0] + 0x20);
            ret(e.mem.u32(array + 4 * a[1]))
        });
        e
    }

    fn form(e: &mut Engine, id: u32) -> u32 {
        let p = e.mem.alloc(0x10);
        e.mem.set_u32(p + 0xc, id);
        p
    }

    /// A per-quest entry with these infos (count at +4+0xc, array at +0x20).
    fn entry(e: &mut Engine, infos: &[u32]) -> u32 {
        let p = e.mem.alloc(0x24);
        e.mem.set_u32(p + 0x10, infos.len() as u32);
        let array = e.mem.alloc(4 * infos.len().max(1) as u32);
        for (i, info) in infos.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *info);
        }
        e.mem.set_u32(p + 0x20, array);
        p
    }

    /// Writes a list at `head` (an embedded node) holding `items`.
    fn list(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            let next = if i + 1 < items.len() {
                e.mem.alloc(8)
            } else {
                0
            };
            e.mem.set_u32(node + 4, next);
            node = next;
        }
    }

    fn topic(e: &mut Engine, entries: &[u32]) -> u32 {
        let t = e.mem.alloc(0x40);
        list(e, t + 0x2c, entries);
        t
    }

    #[test]
    fn get_topic_checks_the_index() {
        let mut e = Engine::new();
        e.map(TOPIC_COUNTS, 0x40);
        e.mem.set_i32(TOPIC_COUNTS + 4, 2);
        let array = e.mem.alloc(0x18);
        e.mem.set_u32(array + 0xc, 0x4444);
        e.mem.set_u32(TOPIC_ARRAYS + 4, array);
        assert_eq!(e.call(0x0061_a2d0, &args![1u32, 1i32]).u32(), 0x4444);
        assert_eq!(e.call(0x0061_a2d0, &args![1u32, 2i32]).u32(), 0);
        assert_eq!(e.call(0x0061_a2d0, &args![1u32, -1i32]).u32(), 0);
    }

    #[test]
    fn finds_an_info_by_id_in_both_directions() {
        let mut e = setup();
        let first = form(&mut e, 0x77);
        let second = form(&mut e, 0x77);
        let other = form(&mut e, 0x55);
        let empty = entry(&mut e, &[]);
        let full = entry(&mut e, &[other, 0, first, second]);
        let t = topic(&mut e, &[empty, full]);
        assert_eq!(e.call(0x0061_9e10, &args![t, 0x77u32, 0u32]).u32(), first);
        assert_eq!(e.call(0x0061_9e10, &args![t, 0x77u32, 1u32]).u32(), second);
        // Only the low byte of the flag word counts.
        assert_eq!(
            e.call(0x0061_9e10, &args![t, 0x77u32, 0x100u32]).u32(),
            first
        );
        assert_eq!(e.call(0x0061_9e10, &args![t, 0x99u32, 1u32]).u32(), 0);
        // An item-less node ends the walk.
        let stop = topic(&mut e, &[0, full]);
        assert_eq!(e.call(0x0061_9e10, &args![stop, 0x77u32, 0u32]).u32(), 0);
    }

    #[test]
    fn owner_topic_is_the_one_holding_the_info() {
        let mut e = setup();
        let info = form(&mut e, 0x77);
        let twin = form(&mut e, 0x77);
        let holds_twin = entry(&mut e, &[twin]);
        let holds_info = entry(&mut e, &[info]);
        let a = topic(&mut e, &[holds_twin]);
        let b = topic(&mut e, &[holds_info]);
        let handler = e.mem.alloc(0x110);
        list(&mut e, handler + 0x108, &[a, b]);
        e.map(TOPIC_LIST_OWNER, 4);
        e.mem.set_u32(TOPIC_LIST_OWNER, handler);
        assert_eq!(e.call(0x0061_a310, &args![info]).u32(), b);
        let lost = form(&mut e, 0x55);
        assert_eq!(e.call(0x0061_a310, &args![lost]).u32(), 0);
    }
}
