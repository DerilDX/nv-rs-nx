//! `(unplaced)/run_00619b60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESTopic::GetTopic` (`0061a2d0`, cdecl: kind, index).
const GET_TOPIC: u32 = 0x0061_a2d0;
/// The kind and index of the topic `00619d00` searches instead of a topic
/// whose flag bit 0 (`00619df0`) is set.
const REDIRECT_KIND: u32 = 1;
const REDIRECT_INDEX: u32 = 5;
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
/// `0084e3a0` (`ECX` = object): the word at +0xc (an array's count).
const WORD_AT_0XC: u32 = 0x0084_e3a0;
/// `00619f20` (cdecl: per-quest entry, index): the info at that index.
const ENTRY_INFO: u32 = 0x0061_9f20;

// Translated from 00619d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The quest under which the topic (`ECX`) holds the topic info (one stack
/// word; `RET 4`): 0 for a null info; for a topic with flag bit 0 set
/// (`00619df0`), the answer of the topic `GetTopic(1, 5)` instead; else the
/// first word (the quest) of the first per-quest entry (`005d8a70`'s list,
/// walked until a node with no item) whose infos (`00619f20`, below the
/// count `0084e3a0` of `00717e50`, unsigned) include the info. 0 if none.
/// The engine map gives it no name.
pub fn fn_00619d00(e: &mut Engine, this: Ptr, info: u32) -> u32 {
    if info == 0 {
        return 0;
    }
    if fn_00619df0(e, this) {
        let topic = e
            .call(GET_TOPIC, &args![REDIRECT_KIND, REDIRECT_INDEX])
            .ptr();
        return fn_00619d00(e, topic, info);
    }
    let mut node = e.call(TOPIC_QUEST_LIST, &args![this]).u32();
    loop {
        if node == 0 {
            return 0;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            return 0;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(item);
        node = e.call(NODE_NEXT, &args![node]).u32();
        let infos = e.call(PLUS_FOUR, &args![entry]).u32();
        let count = e.call(WORD_AT_0XC, &args![infos]).u32();
        let mut index = 0u32;
        while index < count {
            if e.call(ENTRY_INFO, &args![entry, index]).u32() == info {
                return e.mem.u32(entry);
            }
            index += 1;
        }
    }
}

// Translated from 00619df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of the topic's (`ECX`) byte at +0x25 is set. The engine
/// map gives it no name.
pub fn fn_00619df0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr().wrapping_add(0x25)) & 1 != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_9d00, fn_00619d00(Ptr, u32) -> u32),
        entry!(0x0061_9df0, fn_00619df0(Ptr) -> bool),
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

    /// Doubles as the exe's bodies: a node's item is its first word, the
    /// next node its second; an entry's info count is at +0x10 and its
    /// infos in an array whose address is at +0x20.
    fn setup() -> Engine {
        let mut e = Engine::new();
        e.register(TOPIC_QUEST_LIST, |_, a| ret(a[0] + 0x2c));
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

    /// A per-quest entry for `quest` with these infos.
    fn entry(e: &mut Engine, quest: u32, infos: &[u32]) -> u32 {
        let p = e.mem.alloc(0x24);
        e.mem.set_u32(p, quest);
        e.mem.set_u32(p + 0x10, infos.len() as u32);
        let array = e.mem.alloc(4 * infos.len().max(1) as u32);
        for (i, info) in infos.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *info);
        }
        e.mem.set_u32(p + 0x20, array);
        p
    }

    /// A topic whose per-quest list (at +0x2c) holds these entries.
    fn topic(e: &mut Engine, entries: &[u32]) -> u32 {
        let t = e.mem.alloc(0x40);
        let mut node = t + 0x2c;
        for (i, item) in entries.iter().enumerate() {
            e.mem.set_u32(node, *item);
            let next = if i + 1 < entries.len() {
                e.mem.alloc(8)
            } else {
                0
            };
            e.mem.set_u32(node + 4, next);
            node = next;
        }
        t
    }

    #[test]
    fn flag_bit_zero() {
        let mut e = Engine::new();
        let t = e.mem.alloc(0x30);
        assert!(!e.call(0x0061_9df0, &args![t]).bool());
        e.mem.set_u8(t + 0x25, 0xfe);
        assert!(!e.call(0x0061_9df0, &args![t]).bool());
        e.mem.set_u8(t + 0x25, 0x81);
        assert!(e.call(0x0061_9df0, &args![t]).bool());
    }

    #[test]
    fn finds_the_quest_holding_the_info() {
        let mut e = setup();
        let first = entry(&mut e, 0x1111, &[0x10, 0x20]);
        let second = entry(&mut e, 0x2222, &[0x30, 0x40]);
        let t = topic(&mut e, &[first, second]);
        assert_eq!(e.call(0x0061_9d00, &args![t, 0x40u32]).u32(), 0x2222);
        assert_eq!(e.call(0x0061_9d00, &args![t, 0x10u32]).u32(), 0x1111);
        assert_eq!(e.call(0x0061_9d00, &args![t, 0x50u32]).u32(), 0);
        assert_eq!(e.call(0x0061_9d00, &args![t, 0u32]).u32(), 0);
    }

    #[test]
    fn a_flagged_topic_searches_topic_one_five() {
        let mut e = setup();
        let held = entry(&mut e, 0x3333, &[0x40]);
        let redirect = topic(&mut e, &[held]);
        e.register_double(GET_TOPIC, move |_, _| ret(redirect));
        let flagged = topic(&mut e, &[]);
        e.mem.set_u8(flagged + 0x25, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_9d00, &args![flagged, 0x40u32]).u32(), 0x3333);
        let log = e.call_log.take().unwrap();
        let topics: Vec<_> = log.iter().filter(|c| c.0 == GET_TOPIC).collect();
        assert_eq!(topics, [&(GET_TOPIC, vec![1, 5])]);
    }
}
