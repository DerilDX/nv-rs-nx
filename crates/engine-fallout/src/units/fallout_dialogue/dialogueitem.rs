//! `fallout/dialogue/dialogueitem.cpp` (Xbox PDB source unit), subsystem `fallout/dialogue`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `DialogueItem` keeps its responses in a `BSSimpleList<DialogueResponse *>` at +0; the
//! list helpers it calls: `006815c0` (the address of a node's item), `00726070` (the next
//! node), `008256d0` (whether a node is an empty list).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `DialogueItem` (Xbox PDB), 0x1c bytes.
    pub struct DialogueItem: 0x1c {
        /// `lResponses` (Xbox PDB): `BSSimpleList<DialogueResponse *>`, inline.
        0x00 lResponses: u32,
        /// `pCurrentResponse` (Xbox PDB): `BSSimpleList<DialogueResponse *>*`.
        0x08 pCurrentResponse: u32,
        /// `pInfo` (Xbox PDB): `TESTopicInfo*`.
        0x0C pInfo: u32,
        /// `pTopic` (Xbox PDB): `TESTopic*`.
        0x10 pTopic: u32,
        /// `pQuest` (Xbox PDB): `TESQuest*`.
        0x14 pQuest: u32,
        /// `pSpeaker` (Xbox PDB): `TESObjectREFR*`.
        0x18 pSpeaker: u32,
    }
}

/// `006815c0` (`thiscall`): the address of a list node's item.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`thiscall`): a list node's next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `008256d0` (`thiscall`): whether a list is empty (`AL`).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `0063f7b0` (`thiscall`): removes a list's first item.
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// The list destructor `0046ffb0` (`thiscall`).
const LIST_DESTRUCTOR: u32 = 0x0046_ffb0;
/// The list constructor `0096a2d0` (`thiscall`).
const LIST_CONSTRUCTOR: u32 = 0x0096_a2d0;
/// `00905820` (`thiscall`, one word: the address of the item): adds an item to a list.
const LIST_ADD: u32 = 0x0090_5820;
/// The response-list iterator constructor `0044dee0`, its current node `00559450`, a
/// node's next node `0045cd60` and the iterator destructor `0061f440` (all `thiscall`).
const ITERATOR_CONSTRUCTOR: u32 = 0x0044_dee0;
const ITERATOR_NODE: u32 = 0x0055_9450;
const NEXT_NODE_0045CD60: u32 = 0x0045_cd60;
const ITERATOR_DESTRUCTOR: u32 = 0x0061_f440;
/// `TESTopicInfo::CloneResponseList` (engine map, `0061e760`, `thiscall`, one word: the
/// iterator to fill).
const CLONE_RESPONSE_LIST: u32 = 0x0061_e760;
/// The allocator `00401000` and the free `00401030` (`cdecl`, one word).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// The `DialogueResponse` constructor `0083d280` (`thiscall`, five words).
const RESPONSE_CONSTRUCTOR: u32 = 0x0083_d280;
/// The `BSStringT<char>` destructor `004037d0` (`thiscall`).
const STRING_DESTRUCTOR: u32 = 0x0040_37d0;
/// `00579200` (`thiscall`), `TESTopicInfo::AddTopicList` (`0061f150`, `thiscall`) and
/// `TESTopicInfo::RunResult` (`0061f170`, `thiscall`, two words).
const INFO_TEST_00579200: u32 = 0x0057_9200;
const ADD_TOPIC_LIST: u32 = 0x0061_f150;
const RUN_RESULT: u32 = 0x0061_f170;
/// `005b5e40` (`cdecl`, one word: a message) with the message at `0107dba8`.
const REPORT_005B5E40: u32 = 0x005b_5e40;
const MISSING_RESPONSE_MESSAGE: u32 = 0x0107_dba8;
/// `DialogueResponse::SaveGame` (`0083d690`, `thiscall`, one word).
const RESPONSE_SAVE_GAME: u32 = 0x0083_d690;
/// `BGSSaveGameBuffer` writers (`thiscall`): `StartVariableSizedValue` (`00865f20`),
/// `SaveVariableSizedValue_ov2` (`00865ff0`, two words), the raw write `00865e50` (three
/// words: address, size, 0) and `SaveFormID_ov2` (`00865df0`, two words).
const START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5ff0;
const SAVE_DATA: u32 = 0x0086_5e50;
const SAVE_FORM_ID: u32 = 0x0086_5df0;

/// The item of a list node (`*006815c0(node)`).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(slot)
}

// Translated from 0083c520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::DialogueItem_ov2` (engine map; `thiscall`, `RET 0x10`: quest, topic,
/// info, speaker): constructs the response list and clears `pCurrentResponse` and `pInfo`.
/// With a topic, an info and a quest it keeps all four and makes one `DialogueResponse`
/// (0x2c bytes, `0083d280` with the four and the info's response node; null when the
/// allocation fails) for each response of the info (an iterator on the stack filled by
/// `TESTopicInfo::CloneResponseList`), adding each to the list. Returns `this`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0083c520 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn dialogue_item_constructor(
    e: &mut Engine,
    this: Ptr<DialogueItem>,
    quest: u32,
    topic: u32,
    info: u32,
    speaker: u32,
) -> Ptr<DialogueItem> {
    e.call(LIST_CONSTRUCTOR, &args![this]);
    e.set(this, DialogueItem::pCurrentResponse, 0);
    e.set(this, DialogueItem::pInfo, 0);
    if topic == 0 || info == 0 || quest == 0 {
        return this;
    }
    e.set(this, DialogueItem::pInfo, info);
    e.set(this, DialogueItem::pTopic, topic);
    e.set(this, DialogueItem::pQuest, quest);
    e.set(this, DialogueItem::pSpeaker, speaker);
    e.with_stack(8, |e, locals| {
        let iterator = locals.addr();
        let item = iterator + 4;
        e.call(ITERATOR_CONSTRUCTOR, &args![iterator]);
        e.call(CLONE_RESPONSE_LIST, &args![info, iterator]);
        let mut node = e.call(ITERATOR_NODE, &args![iterator]).u32();
        while node != 0 {
            let block = e.call(ALLOCATE, &args![0x2cu32]).u32();
            let response = if block != 0 {
                e.call(
                    RESPONSE_CONSTRUCTOR,
                    &args![block, quest, topic, info, speaker, node],
                )
                .u32()
            } else {
                0
            };
            e.mem.set_u32(item, response);
            e.call(LIST_ADD, &args![this, item]);
            node = e.call(NEXT_NODE_0045CD60, &args![node]).u32();
        }
        e.call(ITERATOR_DESTRUCTOR, &args![iterator]);
    });
    this
}

// Translated from 0083c670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::~DialogueItem` (engine map; `thiscall`): while the response list is not
/// empty, deletes its first response ([`fn_0083c720`] with 1, when not null) and removes
/// it (`0063f7b0`); then destroys the list (`0046ffb0`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0083c670 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn dialogue_item_destructor(e: &mut Engine, this: Ptr<DialogueItem>) {
    let list = this.addr();
    while list != 0 && e.call(LIST_IS_EMPTY, &args![list]).u8() == 0 {
        let response = node_item(e, list);
        if response != 0 {
            fn_0083c720(e, Ptr::new(response), 1);
        }
        e.call(LIST_REMOVE_HEAD, &args![list]);
    }
    e.call(LIST_DESTRUCTOR, &args![this]);
}

// Translated from 0083c720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `DialogueResponse` scalar deleting destructor (unnamed in the engine map;
/// `thiscall`, `RET 4`: the flags): runs [`fn_0083c750`] and frees the response
/// (`00401030`) when bit 0 of the flags is set. Returns `this`.
pub fn fn_0083c720(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0083c750(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0083c750 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `DialogueResponse` destructor body (unnamed in the engine map; `thiscall`):
/// destroys `cVoice` (+0x10) and then `cText` (+0) (`BSStringT<char>`, `004037d0`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0083c750 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_0083c750(e: &mut Engine, this: Ptr) {
    e.call(STRING_DESTRUCTOR, &args![this.addr() + 0x10]);
    e.call(STRING_DESTRUCTOR, &args![this]);
}

// Translated from 0083c7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::FirstResponse` (engine map; `thiscall`): points `pCurrentResponse` at
/// the list's first node and answers whether it holds a response.
pub fn dialogue_item_first_response(e: &mut Engine, this: Ptr<DialogueItem>) -> bool {
    e.set(this, DialogueItem::pCurrentResponse, this.addr());
    let current = e.get(this, DialogueItem::pCurrentResponse);
    node_item(e, current) != 0
}

// Translated from 0083c7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::NextResponse` (engine map; `thiscall`): moves `pCurrentResponse` to the
/// next node (when it is set) and answers whether it is still set.
pub fn dialogue_item_next_response(e: &mut Engine, this: Ptr<DialogueItem>) -> bool {
    let current = e.get(this, DialogueItem::pCurrentResponse);
    if current != 0 {
        let next = e.call(NODE_NEXT, &args![current]).u32();
        e.set(this, DialogueItem::pCurrentResponse, next);
    }
    e.get(this, DialogueItem::pCurrentResponse) != 0
}

// Translated from 0083c820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::GetCurrentResponse` (engine map; `thiscall`): the response of
/// `pCurrentResponse`, or null when it is not set.
pub fn dialogue_item_get_current_response(e: &mut Engine, this: Ptr<DialogueItem>) -> u32 {
    let current = e.get(this, DialogueItem::pCurrentResponse);
    if current == 0 {
        return 0;
    }
    node_item(e, current)
}

// Translated from 0083c850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::RunResult` (engine map; `thiscall`, `RET 4`: a word passed on): when there
/// is an info and its `00579200` answers false, adds its topic list
/// (`TESTopicInfo::AddTopicList`) and runs its result script (`TESTopicInfo::RunResult`
/// with the word and `pSpeaker`).
pub fn dialogue_item_run_result(e: &mut Engine, this: Ptr<DialogueItem>, value: u32) {
    let info = e.get(this, DialogueItem::pInfo);
    if info == 0 || e.call(INFO_TEST_00579200, &args![info]).u8() != 0 {
        return;
    }
    e.call(ADD_TOPIC_LIST, &args![info]);
    let speaker = e.get(this, DialogueItem::pSpeaker);
    let info = e.get(this, DialogueItem::pInfo);
    e.call(RUN_RESULT, &args![info, value, speaker]);
}

// Translated from 0083c900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::GetDialogueResponseIndex` (engine map; `thiscall`, `RET 4`: a response):
/// the 16-bit index of the response in the list, counting from 0 until the list ends or
/// an empty node is reached. A response not found reports the message at `0107dba8`
/// (`005b5e40`) and answers 0.
pub fn dialogue_item_get_dialogue_response_index(
    e: &mut Engine,
    this: Ptr<DialogueItem>,
    response: u32,
) -> u16 {
    let mut index = 0u16;
    let mut node = this.addr();
    while node != 0 && e.call(LIST_IS_EMPTY, &args![node]).u8() == 0 {
        if node_item(e, node) == response {
            return index;
        }
        index = index.wrapping_add(1);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(REPORT_005B5E40, &args![MISSING_RESPONSE_MESSAGE]);
    0
}

// Translated from 0083ce40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::SaveGame_ov2` (engine map; `thiscall`, `RET 4`: the save buffer): saves
/// every non-null response (`DialogueResponse::SaveGame`) as one variable-sized value of
/// their count, then the 16-bit index of the current response (-1 without one; two bytes)
/// and the form ids of `pInfo`, `pTopic`, `pQuest` and `pSpeaker`.
pub fn dialogue_item_save_game(e: &mut Engine, this: Ptr<DialogueItem>, buffer: u32) {
    let mut count = 0u32;
    let mark = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let mut node = this.addr();
    while node != 0 {
        let response = node_item(e, node);
        if response != 0 {
            e.call(RESPONSE_SAVE_GAME, &args![response, buffer]);
            count = count.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count, mark]);
    let current = e.get(this, DialogueItem::pCurrentResponse);
    let index = if current != 0 {
        let response = node_item(e, current);
        // CWDE: the index sign-extended.
        dialogue_item_get_dialogue_response_index(e, this, response) as i16 as i32
    } else {
        -1
    };
    e.with_stack(4, |e, word| {
        e.mem.set_u16(word.addr(), index as u16);
        e.call(SAVE_DATA, &args![buffer, word, 2u32, 0u32]);
    });
    for field in [
        DialogueItem::pInfo,
        DialogueItem::pTopic,
        DialogueItem::pQuest,
        DialogueItem::pSpeaker,
    ] {
        let form = e.get(this, field);
        e.call(SAVE_FORM_ID, &args![buffer, form, 0u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0083_c520,
            dialogue_item_constructor(Ptr<DialogueItem>, u32, u32, u32, u32) -> Ptr<DialogueItem>
        ),
        entry!(0x0083_c670, dialogue_item_destructor(Ptr<DialogueItem>)),
        entry!(0x0083_c720, fn_0083c720(Ptr, u32) -> Ptr),
        entry!(0x0083_c750, fn_0083c750(Ptr)),
        entry!(
            0x0083_c7b0,
            dialogue_item_first_response(Ptr<DialogueItem>) -> bool
        ),
        entry!(
            0x0083_c7e0,
            dialogue_item_next_response(Ptr<DialogueItem>) -> bool
        ),
        entry!(
            0x0083_c820,
            dialogue_item_get_current_response(Ptr<DialogueItem>) -> u32
        ),
        entry!(
            0x0083_c850,
            dialogue_item_run_result(Ptr<DialogueItem>, u32)
        ),
        entry!(
            0x0083_c900,
            dialogue_item_get_dialogue_response_index(Ptr<DialogueItem>, u32) -> u16
        ),
        entry!(0x0083_ce40, dialogue_item_save_game(Ptr<DialogueItem>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn called(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == target)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// List doubles over plain `BSSimpleList` nodes (item, next): the item's address is the
    /// node, the next node is +4, a list is empty when its item and next are both 0.
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    /// A list at `head` holding `items` (extra nodes are fresh blocks).
    fn fill(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
    }

    #[test]
    fn constructor_makes_a_response_per_info_response() {
        let mut e = list_engine();
        e.register(LIST_CONSTRUCTOR, |_, _| Ret::default());
        e.register(ITERATOR_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(CLONE_RESPONSE_LIST, |e, a| {
            e.mem.set_u32(a[1], 0x0000_0100);
            Ret::default()
        });
        e.register(ITERATOR_NODE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(NEXT_NODE_0045CD60, |_, a| {
            let next = if a[0] < 0x300 { a[0] + 0x100 } else { 0 };
            next.into_ret()
        });
        e.register(ITERATOR_DESTRUCTOR, |_, _| Ret::default());
        e.register(ALLOCATE, |_, _| 0x5000u32.into_ret());
        e.register(RESPONSE_CONSTRUCTOR, |_, a| (a[0] + a[5]).into_ret());
        e.register(LIST_ADD, |e, a| {
            assert_ne!(e.mem.u32(a[1]), 0);
            Ret::default()
        });
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 8, 0xee);
        assert_eq!(
            e.call(0x0083_c520, &[this, 0x14, 0x10, 0xc, 0x18]).u32(),
            this
        );
        for (offset, value) in [
            (8u32, 0u32),
            (0xc, 0xc),
            (0x10, 0x10),
            (0x14, 0x14),
            (0x18, 0x18),
        ] {
            assert_eq!(e.mem.u32(this + offset), value);
        }
        let made = called(&e, RESPONSE_CONSTRUCTOR);
        let nodes: Vec<u32> = made.iter().map(|w| w[5]).collect();
        assert_eq!(nodes, vec![0x100, 0x200, 0x300]);
        assert_eq!(made[0][1..5], [0x14, 0x10, 0xc, 0x18]);
        assert_eq!(called(&e, LIST_ADD).len(), 3);
        assert_eq!(called(&e, ITERATOR_DESTRUCTOR).len(), 1);

        // Without a quest only the list is made.
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        e.call(0x0083_c520, &[this, 0, 0x10, 0xc, 0x18]);
        assert!(called(&e, CLONE_RESPONSE_LIST).is_empty());
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn destructor_deletes_every_response() {
        let mut e = list_engine();
        e.register(LIST_REMOVE_HEAD, |e, a| {
            // Pop the head: copy the next node over it.
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
        e.register(LIST_DESTRUCTOR, |_, _| Ret::default());
        e.register(STRING_DESTRUCTOR, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        let a = e.mem.alloc(0x2c);
        let b = e.mem.alloc(0x2c);
        fill(&mut e, this, &[a, 0, b]);
        e.call(0x0083_c670, &[this]);
        let freed: Vec<u32> = called(&e, FREE).iter().map(|w| w[0]).collect();
        assert_eq!(freed, vec![a, b]);
        assert_eq!(called(&e, LIST_REMOVE_HEAD).len(), 3);
        assert_eq!(called(&e, LIST_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn response_destructors_destroy_both_strings() {
        let mut e = list_engine();
        e.register(STRING_DESTRUCTOR, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        e.call(0x0083_c750, &[0x1000]);
        assert_eq!(
            called(&e, STRING_DESTRUCTOR),
            vec![vec![0x1010], vec![0x1000]]
        );
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(0x0083_c720, &[0x1000, flags]).u32(), 0x1000);
        }
        assert_eq!(called(&e, STRING_DESTRUCTOR).len(), 8);
        assert_eq!(called(&e, FREE), vec![vec![0x1000], vec![0x1000]]);
    }

    #[test]
    fn first_next_and_current_walk_the_list() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x1c);
        fill(&mut e, this, &[0x11, 0x22]);
        assert!(e.call(0x0083_c7b0, &[this]).bool());
        assert_eq!(e.mem.u32(this + 8), this);
        assert_eq!(e.call(0x0083_c820, &[this]).u32(), 0x11);
        assert!(e.call(0x0083_c7e0, &[this]).bool());
        assert_eq!(e.call(0x0083_c820, &[this]).u32(), 0x22);
        assert!(!e.call(0x0083_c7e0, &[this]).bool());
        assert_eq!(e.call(0x0083_c820, &[this]).u32(), 0);
        assert!(!e.call(0x0083_c7e0, &[this]).bool());
        // An empty list's first node holds nothing.
        let empty = e.mem.alloc(0x1c);
        assert!(!e.call(0x0083_c7b0, &[empty]).bool());
    }

    #[test]
    fn run_result_needs_an_info_that_answers_false() {
        let mut e = list_engine();
        e.register(INFO_TEST_00579200, |_, a| {
            u32::from(a[0] == 0x2000).into_ret()
        });
        e.register(ADD_TOPIC_LIST, |_, _| Ret::default());
        e.register(RUN_RESULT, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x18, 0x18);
        e.call(0x0083_c850, &[this, 7]);
        assert!(called(&e, INFO_TEST_00579200).is_empty());
        e.mem.set_u32(this + 0xc, 0x2000);
        e.call(0x0083_c850, &[this, 7]);
        assert!(called(&e, RUN_RESULT).is_empty());
        e.mem.set_u32(this + 0xc, 0x1000);
        e.call(0x0083_c850, &[this, 7]);
        assert_eq!(called(&e, ADD_TOPIC_LIST), vec![vec![0x1000]]);
        assert_eq!(called(&e, RUN_RESULT), vec![vec![0x1000, 7, 0x18]]);
    }

    #[test]
    fn response_index_counts_or_reports() {
        let mut e = list_engine();
        e.register(REPORT_005B5E40, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        fill(&mut e, this, &[0x11, 0x22, 0x33]);
        assert_eq!(e.call(0x0083_c900, &[this, 0x33]).u16(), 2);
        assert_eq!(e.call(0x0083_c900, &[this, 0x11]).u16(), 0);
        assert!(called(&e, REPORT_005B5E40).is_empty());
        assert_eq!(e.call(0x0083_c900, &[this, 0x44]).u32(), 0);
        assert_eq!(
            called(&e, REPORT_005B5E40),
            vec![vec![MISSING_RESPONSE_MESSAGE]]
        );
    }

    #[test]
    fn save_game_writes_responses_index_and_forms() {
        let mut e = list_engine();
        e.register(START_VARIABLE_SIZED_VALUE, |_, _| 0x77u32.into_ret());
        e.register(RESPONSE_SAVE_GAME, |_, _| Ret::default());
        e.register(SAVE_VARIABLE_SIZED_VALUE, |_, _| Ret::default());
        e.register(SAVE_DATA, |e, a| {
            // Records the two bytes written.
            let written = u32::from(e.mem.u16(a[1]));
            e.mem.set_u32(0x0090_0000, written);
            Ret::default()
        });
        e.register(SAVE_FORM_ID, |_, _| Ret::default());
        e.map(0x0090_0000, 0x10);
        let this = e.mem.alloc(0x1c);
        fill(&mut e, this, &[0x11, 0, 0x33]);
        for (offset, value) in [(0xcu32, 0xc0u32), (0x10, 0x10), (0x14, 0x14), (0x18, 0x18)] {
            e.mem.set_u32(this + offset, value);
        }
        // The current response is the third node's.
        let second = e.mem.u32(this + 4);
        let third = e.mem.u32(second + 4);
        e.mem.set_u32(this + 8, third);
        e.call(0x0083_ce40, &[this, 0xb0]);
        let saved: Vec<u32> = called(&e, RESPONSE_SAVE_GAME)
            .iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(saved, vec![0x11, 0x33]);
        assert_eq!(
            called(&e, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![0xb0, 2, 0x77]]
        );
        assert_eq!(e.mem.u32(0x0090_0000), 2);
        let data = called(&e, SAVE_DATA);
        assert_eq!(data[0][2..], [2, 0]);
        let forms: Vec<u32> = called(&e, SAVE_FORM_ID).iter().map(|w| w[1]).collect();
        assert_eq!(forms, vec![0xc0, 0x10, 0x14, 0x18]);

        // No current response: -1.
        e.mem.set_u32(this + 8, 0);
        e.call(0x0083_ce40, &[this, 0xb0]);
        assert_eq!(e.mem.u32(0x0090_0000), 0xffff);
    }
}
