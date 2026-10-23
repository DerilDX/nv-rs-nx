//! `fallout/dialogue/conversation.cpp` (Xbox PDB source unit), subsystem `fallout/dialogue`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `Conversation` (Xbox PDB, 0xC bytes): `lDialogueItems`
//! (`BSSimpleList<DialogueItem *>`, +0, first node embedded: item +0,
//! next +4) and `pCurrentItem` (+8, the node of the current item). The
//! list is walked as the code walks it: `006815c0` answers the node, whose
//! item is its first word, `00726070` the next node, `008256d0` whether a
//! node is empty (item and next both 0). The compiler's exception frames
//! (`FS:[0]`) are not translated; the functions that have them carry a
//! `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;

/// `pCurrentItem` (Xbox PDB).
const CURRENT_ITEM: u32 = 0x8;

/// The `BSSimpleList` constructor (`0096a2d0`, thiscall: clears item and next).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `BSSimpleList` destructor (thiscall).
const LIST_DESTROY: u32 = 0x0046_ffb0;
/// `TESTopic::CreateConversation` (engine map), cdecl (four words).
const TOPIC_CREATE_CONVERSATION: u32 = 0x0061_b440;
/// thiscall on a list node: whether it is empty.
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// thiscall on a list node: answers the node itself.
const LIST_NODE_ITEM: u32 = 0x0068_15c0;
/// thiscall on a list node: the next node (+4).
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// thiscall on a list: removes the head node (`0063f7b0`).
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// The `DialogueItem` deleting destructor (`005c90d0`, thiscall (item, flags)).
const DIALOGUE_ITEM_DELETING_DESTRUCTOR: u32 = 0x005c_90d0;
/// cdecl (text): prints a message.
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// The message `GetDialogueItemIndex` prints when the item is not in the list.
const ITEM_NOT_FOUND_MESSAGE: u32 = 0x0107_da30;
/// `DialogueItem::SaveGame_ov2` (engine map), thiscall (item, buffer).
const DIALOGUE_ITEM_SAVE_GAME: u32 = 0x0083_ce40;
/// `BGSSaveGameBuffer::StartVariableSizedValue` (engine map), thiscall (buffer).
const SAVE_START_VARIABLE_SIZED: u32 = 0x0086_5f20;
/// `BGSSaveGameBuffer::SaveVariableSizedValue_ov2` (engine map), thiscall (buffer, value, mark).
const SAVE_VARIABLE_SIZED: u32 = 0x0086_5ff0;
/// thiscall (buffer, data, size, 0): writes bytes to the save buffer.
const SAVE_BUFFER_WRITE: u32 = 0x0086_5e50;
/// thiscall on a `DialogueItem`'s response list: precaches its sounds (`0083d130`).
const PRECACHE_RESPONSES: u32 = 0x0083_d130;

fn node_item(e: &mut Engine, node: u32) -> u32 {
    let at = e.call(LIST_NODE_ITEM, &args![node]).u32();
    e.mem.u32(at)
}

fn node_is_empty(e: &mut Engine, node: u32) -> bool {
    e.call(LIST_NODE_IS_EMPTY, &args![node]).bool()
}

// Translated from 0083b850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083b850 - the C++ exception-unwinding frame (FS:[0]) that destroys the list when the topic call throws is not translated
/// `Conversation::Conversation_ov2` (Xbox PDB), thiscall `(topic, speaker,
/// third)`: constructs the item list, clears `pCurrentItem` and fills the
/// conversation through `TESTopic::CreateConversation(topic, speaker,
/// this, third)`. Answers `this`.
pub fn conversation_conversation_ov2(
    e: &mut Engine,
    this: Ptr,
    topic: u32,
    speaker: u32,
    third: u32,
) -> Ptr {
    let base = this.addr();
    e.call(LIST_CONSTRUCT, &args![base]);
    e.mem.set_u32(base + CURRENT_ITEM, 0);
    e.call(
        TOPIC_CREATE_CONVERSATION,
        &args![topic, speaker, base, third],
    );
    this
}

// Translated from 0083b8d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083b8d0 - the C++ exception-unwinding frame (FS:[0]) that destroys the list when the clear throws is not translated
/// `Conversation::~Conversation` (unnamed in the engine map; the body is
/// the destructor): deletes the items (`0083b930`) and destroys the list.
pub fn fn_0083b8d0(e: &mut Engine, this: Ptr) {
    fn_0083b930(e, this);
    e.call(LIST_DESTROY, &args![this]);
}

// Translated from 0083b930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes every dialogue item (unnamed in the engine map): while the list
/// is not empty, deletes the head item (its deleting destructor with 1)
/// when there is one and removes the head node; then clears
/// `pCurrentItem`.
pub fn fn_0083b930(e: &mut Engine, this: Ptr) {
    let list = this.addr();
    while list != 0 && !node_is_empty(e, list) {
        let item = node_item(e, list);
        if item != 0 {
            e.call(DIALOGUE_ITEM_DELETING_DESTRUCTOR, &args![item, 1u32]);
        }
        e.call(LIST_REMOVE_HEAD, &args![list]);
    }
    e.mem.set_u32(list + CURRENT_ITEM, 0);
}

// Translated from 0083b9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Conversation::FirstItem` (Xbox PDB): makes the first node current
/// (`pCurrentItem = this`) and answers whether it holds an item.
pub fn conversation_first_item(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    e.mem.set_u32(base + CURRENT_ITEM, base);
    let current = e.mem.u32(base + CURRENT_ITEM);
    if current == 0 {
        return false;
    }
    let current = e.mem.u32(base + CURRENT_ITEM);
    node_item(e, current) != 0
}

// Translated from 0083ba50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Conversation::GetDialogueItemIndex` (Xbox PDB), thiscall `(item)`: the
/// index (16 bits, in `AX`) of the item in the list, counting until an
/// empty node; when it is not found, prints the message at `0107da30` and
/// answers 0.
pub fn conversation_get_dialogue_item_index(e: &mut Engine, this: Ptr, item: u32) -> u16 {
    let mut index = 0u16;
    let mut node = this.addr();
    while node != 0 && !node_is_empty(e, node) {
        if item == node_item(e, node) {
            return index;
        }
        index = index.wrapping_add(1);
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    e.call(PRINT_MESSAGE, &args![ITEM_NOT_FOUND_MESSAGE]);
    0
}

// Translated from 0083c270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Conversation::SaveGame` (Xbox PDB), thiscall `(buffer)`: saves every
/// item of the list (each node with an item) as a variable-sized count,
/// then the current item's index as two bytes (-1 when there is no current
/// item).
pub fn conversation_save_game(e: &mut Engine, this: Ptr, buffer: u32) {
    let base = this.addr();
    let mut count = 0u32;
    let mark = e.call(SAVE_START_VARIABLE_SIZED, &args![buffer]).u32();
    let mut node = base;
    while node != 0 {
        let item = node_item(e, node);
        if item != 0 {
            e.call(DIALOGUE_ITEM_SAVE_GAME, &args![item, buffer]);
            count += 1;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    e.call(SAVE_VARIABLE_SIZED, &args![buffer, count, mark]);
    let index = if e.mem.u32(base + CURRENT_ITEM) != 0 {
        let current = e.mem.u32(base + CURRENT_ITEM);
        let item = node_item(e, current);
        conversation_get_dialogue_item_index(e, this, item)
    } else {
        0xffff
    };
    e.with_stack(4, |e, value| {
        e.mem.set_u16(value.addr(), index);
        e.call(SAVE_BUFFER_WRITE, &args![buffer, value, 2u32, 0u32]);
    });
}

// Translated from 0083c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Conversation::PrecacheSounds` (Xbox PDB): for each item of the list
/// (until an empty node), precaches its responses' sounds (`0083d130` on
/// the item).
pub fn conversation_precache_sounds(e: &mut Engine, this: Ptr) {
    let mut node = this.addr();
    while node != 0 && !node_is_empty(e, node) {
        if node != 0 && node_item(e, node) != 0 {
            let item = node_item(e, node);
            e.call(PRECACHE_RESPONSES, &args![item]);
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0083b850,
            conversation_conversation_ov2(Ptr, u32, u32, u32) -> Ptr
        ),
        entry!(0x0083b8d0, fn_0083b8d0(Ptr)),
        entry!(0x0083b930, fn_0083b930(Ptr)),
        entry!(0x0083b9a0, conversation_first_item(Ptr) -> bool),
        entry!(
            0x0083ba50,
            conversation_get_dialogue_item_index(Ptr, u32) -> u16
        ),
        entry!(0x0083c270, conversation_save_game(Ptr, u32)),
        entry!(0x0083c470, conversation_precache_sounds(Ptr)),
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

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An engine with the list helpers doubled as the game's.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(LIST_NODE_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
        e
    }

    /// A conversation whose list holds `items` (first node embedded).
    fn conversation(e: &mut Engine, items: &[u32]) -> u32 {
        let this = e.mem.alloc(0xc);
        let mut node = this;
        for (index, &item) in items.iter().enumerate() {
            e.mem.set_u32(node, item);
            if index + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        this
    }

    #[test]
    fn constructor_builds_the_list_through_the_topic() {
        let mut e = engine();
        e.register(LIST_CONSTRUCT, |_, _| Ret::default());
        e.register(TOPIC_CREATE_CONVERSATION, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 5);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0083_b850, &args![this, 1u32, 2u32, 3u32]).u32(),
            this
        );
        assert_eq!(calls(&e, LIST_CONSTRUCT), vec![vec![this]]);
        assert_eq!(
            calls(&e, TOPIC_CREATE_CONVERSATION),
            vec![vec![1, 2, this, 3]]
        );
        assert_eq!(e.mem.u32(this + 8), 0);
    }

    #[test]
    fn destructor_deletes_each_item_and_the_list() {
        let mut e = engine();
        e.register(DIALOGUE_ITEM_DELETING_DESTRUCTOR, |_, _| Ret::default());
        e.register(LIST_DESTROY, |_, _| Ret::default());
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let (item, after) = if next != 0 {
                (e.mem.u32(next), e.mem.u32(next + 4))
            } else {
                (0, 0)
            };
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        let this = conversation(&mut e, &[0x10, 0, 0x30]);
        e.mem.set_u32(this + 8, this);
        e.call_log = Some(vec![]);
        e.call(0x0083_b8d0, &args![this]);
        assert_eq!(
            calls(&e, DIALOGUE_ITEM_DELETING_DESTRUCTOR),
            vec![vec![0x10, 1], vec![0x30, 1]]
        );
        assert_eq!(calls(&e, LIST_REMOVE_HEAD).len(), 3);
        assert_eq!(calls(&e, LIST_DESTROY), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 8), 0);
    }

    #[test]
    fn first_item_makes_the_head_current() {
        let mut e = engine();
        let this = conversation(&mut e, &[0x10]);
        assert!(e.call(0x0083_b9a0, &args![this]).bool());
        assert_eq!(e.mem.u32(this + 8), this);
        let empty = conversation(&mut e, &[0]);
        assert!(!e.call(0x0083_b9a0, &args![empty]).bool());
    }

    #[test]
    fn item_index_counts_nodes_and_reports_a_missing_item() {
        let mut e = engine();
        e.register(PRINT_MESSAGE, |_, _| Ret::default());
        let this = conversation(&mut e, &[0x10, 0x20, 0x30]);
        assert_eq!(e.call(0x0083_ba50, &args![this, 0x30u32]).u32(), 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_ba50, &args![this, 0x40u32]).u32(), 0);
        assert_eq!(calls(&e, PRINT_MESSAGE), vec![vec![ITEM_NOT_FOUND_MESSAGE]]);
    }

    #[test]
    fn save_game_saves_the_items_and_the_current_index() {
        let mut e = engine();
        e.register(SAVE_START_VARIABLE_SIZED, |_, _| ret(0x77));
        e.register(DIALOGUE_ITEM_SAVE_GAME, |_, _| Ret::default());
        e.register(SAVE_VARIABLE_SIZED, |_, _| Ret::default());
        let written = std::rc::Rc::new(std::cell::Cell::new(0u16));
        let seen = written.clone();
        e.register_double(SAVE_BUFFER_WRITE, move |e, a| {
            seen.set(e.mem.u16(a[1]));
            Ret::default()
        });
        let this = conversation(&mut e, &[0x10, 0, 0x30]);
        let third = e.mem.u32(e.mem.u32(this + 4) + 4);
        e.mem.set_u32(this + 8, third);
        e.call_log = Some(vec![]);
        e.call(0x0083_c270, &args![this, 0x900u32]);
        assert_eq!(
            calls(&e, DIALOGUE_ITEM_SAVE_GAME),
            vec![vec![0x10, 0x900], vec![0x30, 0x900]]
        );
        assert_eq!(calls(&e, SAVE_VARIABLE_SIZED), vec![vec![0x900, 2, 0x77]]);
        assert_eq!(calls(&e, SAVE_BUFFER_WRITE)[0][2..], [2, 0]);
        assert_eq!(written.get(), 2);
        e.mem.set_u32(this + 8, 0);
        e.call(0x0083_c270, &args![this, 0x900u32]);
        assert_eq!(written.get(), 0xffff);
    }

    #[test]
    fn precache_sounds_visits_each_item() {
        let mut e = engine();
        e.register(PRECACHE_RESPONSES, |_, _| Ret::default());
        let this = conversation(&mut e, &[0x10, 0, 0x30]);
        e.call_log = Some(vec![]);
        e.call(0x0083_c470, &args![this]);
        assert_eq!(calls(&e, PRECACHE_RESPONSES), vec![vec![0x10], vec![0x30]]);
    }
}
