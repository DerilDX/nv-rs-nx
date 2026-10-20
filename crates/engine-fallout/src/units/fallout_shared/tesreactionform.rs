//! `fallout shared/tesreactionform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0048baf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0048bb10` (Ghidra name
/// `TESReactionForm::FreeReactionList`), which empties the reaction list node by node. The
/// Xbox PDB name `TESReactionForm::ClearDataComponent` is the class's vslot +0x004 (pdbdump
/// `type ... TESReactionForm`; engine map row `vt 822dd550`), and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `0048bb10`'s disassembly is a loop that ends on its exit test
/// without loading a result (Ghidra types it void).
pub fn tes_reaction_form_clear_data_component(e: &mut Engine, this: Ptr) {
    e.call(0x0048_bb10, &args![this]);
}

/// `00717e50` (`ECX` = form): the address of the reaction list (`this + 4`;
/// the engine map's name for this folded body is
/// `BaseProcess::GetActorPackageThatIsRunning`).
const REACTION_LIST: u32 = 0x0071_7e50;
/// `006815c0` (`ECX` = list node): the address of the node's item.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`ECX` = list node): the next node, or 0.
const NODE_NEXT: u32 = 0x0072_6070;
/// `0063f7b0` (`ECX` = list node): removes the node's item.
const NODE_REMOVE: u32 = 0x0063_f7b0;
/// `00905330` (`ECX` = list or node, the address of an item): removes that
/// item.
const LIST_REMOVE: u32 = 0x0090_5330;
/// `00905820` (`ECX` = list, the address of an item): adds the item.
const LIST_ADD: u32 = 0x0090_5820;
/// `00401000` (`cdecl`, size): the game's allocation.
const ALLOCATE: u32 = 0x0040_1000;
/// `00401030` (`cdecl`, block): the game's free.
const FREE: u32 = 0x0040_1030;
/// `"MASTERFILE: Form (%08X) not found for reaction of pForm (%08X) '%s'."`
const MISSING_REACTION_FORM: u32 = 0x0101_d480;

/// The address of the reaction list, through `00717e50` as the exe gets it.
fn reaction_list(e: &mut Engine, this: Ptr) -> u32 {
    e.call(REACTION_LIST, &args![this]).u32()
}

/// `*006815c0(node)`: the node's item.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let at = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(at)
}

// Translated from 0048bad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESReactionForm::InitializeDataComponent` (Xbox PDB): the byte at +0xC
/// becomes 8.
pub fn tes_reaction_form_initialize_data_component(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0xc, 8);
}

// Translated from 0048bb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESReactionForm::FreeReactionList` (Xbox PDB): while the list's first
/// item is not null, frees it (`00401030`) and removes it from the list
/// (`0063f7b0` on the list).
pub fn tes_reaction_form_free_reaction_list(e: &mut Engine, this: Ptr) {
    loop {
        let list = reaction_list(e, this);
        if node_item(e, list) == 0 {
            break;
        }
        let list = reaction_list(e, this);
        let item = node_item(e, list);
        e.call(FREE, &args![item]);
        let list = reaction_list(e, this);
        e.call(NODE_REMOVE, &args![list]);
    }
}

// Translated from 0048bc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESReactionForm::InitItem` (Xbox PDB): turns each reaction's form id
/// (the item's first word, given the compile index of `owner`'s file,
/// `TESForm::GetFile(-1)`, or of no file when `owner` is null) into the
/// form (`004839c0`). A missing form is reported ("MASTERFILE: Form (%08X)
/// not found for reaction of pForm (%08X) '%s'.", with the id, `owner`'s
/// form id and name, vtable slot 0x130) and its item removed and freed:
/// through `0063f7b0` on the node when the node has a next node or no
/// earlier node is known, otherwise through `00905330` on the earlier node
/// (with the address of the item), the walk going on from that node's
/// next. The exe never records an earlier node (its local stays 0), so the
/// second way is not taken; it is translated as the exe has it.
pub fn tes_reaction_form_init_item(e: &mut Engine, this: Ptr, owner: Ptr) {
    let mut node = reaction_list(e, this);
    let previous = 0u32;
    let file = if owner.is_null() {
        0
    } else {
        e.call(0x0048_4e60, &args![owner, u32::MAX]).u32()
    };
    e.with_stack(8, |e, locals| {
        let id_at = locals.addr();
        let item_at = locals.addr() + 4;
        while node != 0 && node_item(e, node) != 0 {
            let item = node_item(e, node);
            e.mem.set_u32(item_at, item);
            let id = e.mem.u32(item);
            e.mem.set_u32(id_at, id);
            e.call(0x0048_5d50, &args![id_at, file]);
            let id = e.mem.u32(id_at);
            let form = e.call(0x0048_39c0, &args![id]).u32();
            e.mem.set_u32(item, form);
            if e.mem.u32(item) != 0 {
                node = e.call(NODE_NEXT, &args![node]).u32();
                continue;
            }
            let name = e.vcall(owner.addr(), 0x130, &[]).u32();
            let owner_id = e.call(0x0084_e3a0, &args![owner]).u32();
            let id = e.mem.u32(id_at);
            e.call(
                0x005b_5e40,
                &args![MISSING_REACTION_FORM, id, owner_id, name],
            );
            let has_next = e.call(NODE_NEXT, &args![node]).u32() != 0;
            if has_next || previous == 0 {
                e.call(NODE_REMOVE, &args![node]);
            } else {
                e.call(LIST_REMOVE, &args![previous, item_at]);
                node = e.call(NODE_NEXT, &args![previous]).u32();
            }
            let freed = e.mem.u32(item_at);
            e.call(FREE, &args![freed]);
        }
    });
}

/// The reaction entry for `form` in the list, or 0.
fn find_reaction(e: &mut Engine, this: Ptr, form: u32) -> u32 {
    let mut node = reaction_list(e, this);
    while node != 0 && node_item(e, node) != 0 {
        let item = node_item(e, node);
        if e.mem.u32(item) == form {
            return item;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    0
}

// Translated from 0048bf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The second word (+4, the modifier) of the reaction to `form`, 0 when
/// `form` is null or has no reaction.
pub fn fn_0048bf50(e: &mut Engine, this: Ptr, form: u32) -> u32 {
    if form == 0 {
        return 0;
    }
    match find_reaction(e, this, form) {
        0 => 0,
        item => e.mem.u32(item + 4),
    }
}

// Translated from 0048c1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The third word (+8) of the reaction to `form`, 0 when `form` is null or
/// has no reaction.
pub fn fn_0048c1b0(e: &mut Engine, this: Ptr, form: u32) -> u32 {
    if form == 0 {
        return 0;
    }
    match find_reaction(e, this, form) {
        0 => 0,
        item => e.mem.u32(item + 8),
    }
}

/// A new 0xC-byte reaction (`00401000`; the exe does not check for null)
/// filled by `fill`, added to the list (`00905820` with the address of a
/// stack copy of the pointer).
fn add_reaction(e: &mut Engine, this: Ptr, fill: impl FnOnce(&mut Engine, u32)) {
    let item = e.call(ALLOCATE, &args![0xcu32]).u32();
    fill(e, item);
    e.with_stack(4, |e, item_at| {
        e.mem.set_u32(item_at.addr(), item);
        let list = reaction_list(e, this);
        e.call(LIST_ADD, &args![list, item_at]);
    });
}

// Translated from 0048bfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the modifier (+4) of the reaction to `form` (nothing for a null
/// `form`). An existing reaction gets `modifier`, unless `modifier` and
/// its +8 word are both 0: then it is removed from the list (`00905330` on
/// `this + 4`, with the address of a stack copy) and freed. Without one, a
/// nonzero `modifier` adds a reaction {`form`, `modifier`} (its +8 word not
/// written).
pub fn fn_0048bfc0(e: &mut Engine, this: Ptr, form: u32, modifier: u32) {
    if form == 0 {
        return;
    }
    let item = find_reaction(e, this, form);
    if item != 0 {
        if modifier == 0 && e.mem.u32(item + 8) == 0 {
            e.with_stack(4, |e, item_at| {
                e.mem.set_u32(item_at.addr(), item);
                e.call(LIST_REMOVE, &args![this.addr() + 4, item_at]);
                let freed = e.mem.u32(item_at.addr());
                e.call(FREE, &args![freed]);
            });
        } else {
            e.mem.set_u32(item + 4, modifier);
        }
        return;
    }
    if modifier != 0 {
        add_reaction(e, this, |e, item| {
            e.mem.set_u32(item, form);
            e.mem.set_u32(item + 4, modifier);
        });
    }
}

// Translated from 0048c220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the third word (+8) of the reaction to `form` (nothing for a null
/// `form`): an existing reaction gets `value`; without one a nonzero
/// `value` adds a reaction {`form`, 0, `value`}.
pub fn fn_0048c220(e: &mut Engine, this: Ptr, form: u32, value: u32) {
    if form == 0 {
        return;
    }
    let item = find_reaction(e, this, form);
    if item != 0 {
        e.mem.set_u32(item + 8, value);
        return;
    }
    if value != 0 {
        add_reaction(e, this, |e, item| {
            e.mem.set_u32(item, form);
            e.mem.set_u32(item + 4, 0);
            e.mem.set_u32(item + 8, value);
        });
    }
}

// Translated from 0048c9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the reactions into the save-game buffer `buffer` when bit 2 of its
/// flags word is set (`00428110` copies the word at +0x17 to a stack word,
/// `004280f0` tests it with 4): a variable-sized value
/// (`BGSSaveGameBuffer::StartVariableSizedValue`, `00865f20`), then for
/// each non-null item its form (`SaveFormID_ov2`, `00865df0`, with 0) and
/// its two words (`00865e50`, 4 bytes each, with 0), and finally the count
/// (`SaveVariableSizedValue_ov2`, `00865ff0`, with the count and the
/// start). The walk starts at `this + 4` itself.
pub fn fn_0048c9d0(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let flagged = e.with_stack(4, |e, flags_at| {
        let flags = e.call(0x0042_8110, &args![buffer, flags_at]).u32();
        e.call(0x0042_80f0, &args![flags, 4u32]).bool()
    });
    if !flagged {
        return;
    }
    let mut count = 0u32;
    let start = e.call(0x0086_5f20, &args![buffer]).u32();
    let mut node = this.addr() + 4;
    while node != 0 {
        let item = node_item(e, node);
        if item != 0 {
            let form = e.mem.u32(item);
            e.call(0x0086_5df0, &args![buffer, form, 0u32]);
            e.call(0x0086_5e50, &args![buffer, item + 4, 4u32, 0u32]);
            e.call(0x0086_5e50, &args![buffer, item + 8, 4u32, 0u32]);
            count = count.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(0x0086_5ff0, &args![buffer, count, start]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_baf0, tes_reaction_form_clear_data_component(Ptr)),
        entry!(
            0x0048_bad0,
            tes_reaction_form_initialize_data_component(Ptr)
        ),
        entry!(0x0048_bb10, tes_reaction_form_free_reaction_list(Ptr)),
        entry!(0x0048_bc30, tes_reaction_form_init_item(Ptr, Ptr)),
        entry!(0x0048_bf50, fn_0048bf50(Ptr, u32) -> u32),
        entry!(0x0048_c1b0, fn_0048c1b0(Ptr, u32) -> u32),
        entry!(0x0048_bfc0, fn_0048bfc0(Ptr, u32, u32)),
        entry!(0x0048_c220, fn_0048c220(Ptr, u32, u32)),
        entry!(0x0048_c9d0, fn_0048c9d0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_0048baf0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0048_bb10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_baf0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0048_bb10), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod b020_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine whose list is a chain of (item, next) nodes, the first at
    /// `this + 4`; removing a node's item moves its successor into it.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(REACTION_LIST, |_, a| ret(a[0] + 4));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(NODE_REMOVE, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        for a in [FREE, LIST_REMOVE, LIST_ADD] {
            e.register(a, |_, _| Ret::default());
        }
        e
    }

    /// A form with a list of `items` (each a 0xC-byte reaction {form, a, b}).
    fn form_with(e: &mut Engine, items: &[[u32; 3]]) -> (Ptr, Vec<u32>) {
        let this = e.mem.alloc(0x10);
        let mut node = this + 4;
        let mut made = vec![];
        for (i, words) in items.iter().enumerate() {
            let item = e.mem.alloc(0xc);
            for (k, w) in words.iter().enumerate() {
                e.mem.set_u32(item + 4 * k as u32, *w);
            }
            e.mem.set_u32(node, item);
            made.push(item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        (Ptr::new(this), made)
    }

    #[test]
    fn b020_0048bad0_sets_the_byte() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x10);
        e.call(0x0048_bad0, &args![p]);
        assert_eq!(e.mem.u8(p + 0xc), 8);
    }

    #[test]
    fn b020_0048bb10_frees_every_item() {
        let mut e = engine();
        let (this, items) = form_with(&mut e, &[[1, 0, 0], [2, 0, 0]]);
        e.call_log = Some(vec![]);
        e.call(0x0048_bb10, &args![this]);
        assert_eq!(calls_to(&e, FREE), vec![vec![items[0]], vec![items[1]]]);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
    }

    #[test]
    fn b020_0048bc30_resolves_and_drops_missing_forms() {
        let mut e = engine();
        e.register(0x0048_4e60, |_, _| ret(0x77));
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| {
            ret(if a[0] & 0xf == 0 { 0 } else { a[0] + 0x10 })
        });
        e.register(0x0084_e3a0, |_, _| ret(0x0400_0001));
        e.register(0x0000_1130, |_, _| ret(0x5000));
        e.register(0x005b_5e40, |_, _| Ret::default());
        e.put_vtable(0x0150_0000, &[0; 0x50]);
        e.mem.set_u32(0x0150_0130, 0x0000_1130);
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner, 0x0150_0000);
        let (this, items) = form_with(&mut e, &[[0x11, 1, 0], [0x20, 2, 0], [0x31, 3, 0]]);
        e.call_log = Some(vec![]);
        e.call(0x0048_bc30, &args![this, owner]);
        assert_eq!(e.mem.u32(items[0]), 0x0100_0021);
        assert_eq!(e.mem.u32(items[2]), 0x0100_0041);
        assert_eq!(
            calls_to(&e, 0x005b_5e40),
            vec![vec![
                MISSING_REACTION_FORM,
                0x0100_0020,
                0x0400_0001,
                0x5000
            ]]
        );
        assert_eq!(calls_to(&e, FREE), vec![vec![items[1]]]);
        assert_eq!(calls_to(&e, NODE_REMOVE).len(), 1);
        assert!(calls_to(&e, LIST_REMOVE).is_empty());
        assert_eq!(calls_to(&e, 0x0048_5d50)[0][1], 0x77);
    }

    #[test]
    fn b020_0048bc30_without_owner_uses_no_file() {
        let mut e = engine();
        e.register(0x0048_5d50, |_, _| Ret::default());
        e.register(0x0048_39c0, |_, a| ret(a[0] + 1));
        let (this, _) = form_with(&mut e, &[[0x11, 1, 0]]);
        e.call_log = Some(vec![]);
        e.call(0x0048_bc30, &args![this, 0u32]);
        assert_eq!(calls_to(&e, 0x0048_5d50)[0][1], 0);
        assert!(calls_to(&e, 0x0048_4e60).is_empty());
    }

    #[test]
    fn b020_0048bf50_and_0048c1b0_read_the_words() {
        let mut e = engine();
        let (this, _) = form_with(&mut e, &[[0x11, 5, 6], [0x22, 7, 8]]);
        assert_eq!(e.call(0x0048_bf50, &args![this, 0x22u32]).u32(), 7);
        assert_eq!(e.call(0x0048_c1b0, &args![this, 0x22u32]).u32(), 8);
        assert_eq!(e.call(0x0048_bf50, &args![this, 0x11u32]).u32(), 5);
        assert_eq!(e.call(0x0048_bf50, &args![this, 0x33u32]).u32(), 0);
        assert_eq!(e.call(0x0048_c1b0, &args![this, 0u32]).u32(), 0);
    }

    #[test]
    fn b020_0048bfc0_sets_removes_and_adds() {
        let mut e = engine();
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        let (this, items) = form_with(&mut e, &[[0x11, 5, 0], [0x22, 7, 8]]);
        e.call_log = Some(vec![]);
        e.call(0x0048_bfc0, &args![this, 0x22u32, 9u32]);
        assert_eq!(e.mem.u32(items[1] + 4), 9);
        e.call(0x0048_bfc0, &args![this, 0x22u32, 0u32]);
        assert_eq!(e.mem.u32(items[1] + 4), 0);
        assert!(calls_to(&e, LIST_REMOVE).is_empty());
        e.call(0x0048_bfc0, &args![this, 0x11u32, 0u32]);
        let removed = calls_to(&e, LIST_REMOVE);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], this.addr() + 4);
        assert_eq!(calls_to(&e, FREE), vec![vec![items[0]]]);
        e.call(0x0048_bfc0, &args![this, 0x33u32, 0u32]);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        e.call(0x0048_bfc0, &args![this, 0x33u32, 4u32]);
        let added = calls_to(&e, LIST_ADD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], this.addr() + 4);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0xc]]);
        e.call(0x0048_bfc0, &args![this, 0u32, 4u32]);
        assert_eq!(calls_to(&e, ALLOCATE).len(), 1);
    }

    #[test]
    fn b020_0048c220_sets_or_adds() {
        let mut e = engine();
        let fresh = e.mem.alloc(0xc);
        e.mem.set_u32(fresh + 4, 0xffff);
        e.register_double(ALLOCATE, move |_, _| ret(fresh));
        let (this, items) = form_with(&mut e, &[[0x11, 5, 6]]);
        e.call_log = Some(vec![]);
        e.call(0x0048_c220, &args![this, 0x11u32, 0u32]);
        assert_eq!(e.mem.u32(items[0] + 8), 0);
        e.call(0x0048_c220, &args![this, 0x44u32, 0u32]);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        e.call(0x0048_c220, &args![this, 0x44u32, 3u32]);
        assert_eq!(
            [e.mem.u32(fresh), e.mem.u32(fresh + 4), e.mem.u32(fresh + 8)],
            [0x44, 0, 3]
        );
        assert_eq!(calls_to(&e, LIST_ADD).len(), 1);
    }

    #[test]
    fn b020_0048c9d0_saves_when_flagged() {
        let mut e = engine();
        e.register(0x0042_8110, |e, a| {
            let flags = e.mem.u32(a[0] + 0x17);
            e.mem.set_u32(a[1], flags);
            ret(a[1])
        });
        e.register(0x0042_80f0, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) & a[1] != 0))
        });
        e.register(0x0086_5f20, |_, _| ret(0x55));
        for a in [0x0086_5df0, 0x0086_5e50, 0x0086_5ff0] {
            e.register(a, |_, _| Ret::default());
        }
        let buffer = e.mem.alloc(0x40);
        let (this, items) = form_with(&mut e, &[[0x11, 5, 6], [0, 0, 0], [0x22, 7, 8]]);
        // The middle node's item pointer is made null.
        let second = e.mem.u32(this.addr() + 8);
        e.mem.set_u32(second, 0);
        e.call_log = Some(vec![]);
        e.call(0x0048_c9d0, &args![this, buffer]);
        assert!(calls_to(&e, 0x0086_5f20).is_empty());
        e.mem.set_u32(buffer + 0x17, 4);
        e.call(0x0048_c9d0, &args![this, buffer]);
        assert_eq!(
            calls_to(&e, 0x0086_5df0),
            vec![vec![buffer, 0x11, 0], vec![buffer, 0x22, 0]]
        );
        assert_eq!(
            calls_to(&e, 0x0086_5e50),
            vec![
                vec![buffer, items[0] + 4, 4, 0],
                vec![buffer, items[0] + 8, 4, 0],
                vec![buffer, items[2] + 4, 4, 0],
                vec![buffer, items[2] + 8, 4, 0],
            ]
        );
        assert_eq!(calls_to(&e, 0x0086_5ff0), vec![vec![buffer, 2, 0x55]]);
    }
}
