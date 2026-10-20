//! `fallout shared/tesspelllist.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0048d150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers `this + 0xC` (`thiscall`, no arguments): the address of a member
/// at `+0xC`. The engine map gives this folded body no name; its many
/// callers use it on different classes (in `TESSpellList` that offset is
/// `leveledSpellList`, Xbox PDB).
pub fn fn_0048d150(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0xc))
}

// Translated from 0048d0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0048d0f0` (Ghidra name
/// `TESSpellList::FreeSpellList`), which empties the spell lists. The Xbox PDB name
/// `TESSpellList::ClearDataComponent` is the class's vslot +0x004 (pdbdump `type ...
/// TESSpellList`; engine map row `vt 822ded00`), and returns nothing. Parameters: one word,
/// `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX` on.
/// `0048d0f0`'s disassembly ends on a loop exit without loading a result (Ghidra types it
/// void).
pub fn tes_spell_list_clear_data_component(e: &mut Engine, this: Ptr) {
    e.call(0x0048_d0f0, &args![this]);
}

/// `00717e50` (thiscall): returns `this + 4`, the spell list (`TESSpellList`
/// +0x04, the first node of its list of spells).
const SPELL_LIST: u32 = 0x0071_7e50;
/// `006815c0` (thiscall on a list node): the address of the node's item word.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (thiscall on a list node): the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `0063f7b0` (thiscall on a list node): removes the node's item (the next
/// item moves into the node).
const NODE_REMOVE: u32 = 0x0063_f7b0;
/// `005f65d0` (thiscall on a list, `RET 4`: item pointer): whether the list
/// holds the item.
const LIST_CONTAINS: u32 = 0x005f_65d0;
/// `005ae3d0` (thiscall on a list, `RET 4`: item pointer): adds the item.
const LIST_ADD: u32 = 0x005a_e3d0;
/// `00905330` (thiscall on a list node, `RET 4`: item pointer): the call the
/// spell list's `InitItem` makes on the previous node when the last node's
/// form is missing (`highprocess.cpp` in the engine map).
const NODE_REMOVE_AFTER: u32 = 0x0090_5330;
/// `00726c60` (thiscall on a list node, `RET 4`: item pointer): stores the item.
const NODE_SET_ITEM: u32 = 0x0072_6c60;
/// The player singleton pointer (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `004181e0` (thiscall on the player): the address its actor base's spell
/// list compares with (the value at the player compared after `+0x7C`).
const PLAYER_BASE_FORM: u32 = 0x0041_81e0;
/// `PlayerCharacter::SpellLearned` (Xbox PDB, `0094c6c0`, thiscall, `RET 4`).
const PLAYER_SPELL_LEARNED: u32 = 0x0094_c6c0;
/// The `TES` singleton pointer (`011c3f2c`).
const TES: u32 = 0x011c_3f2c;
/// `004516b0` (thiscall on `TES`, `tes.cpp`): the flag that makes `AddSpell`
/// skip the cast and the player notification.
const TES_FLAG: u32 = 0x0045_16b0;
/// `TESSpellList::AddSpell_ov2` (Xbox PDB, `0048d210`, thiscall, `RET 4`):
/// adds a leveled spell.
const ADD_LEVELED_SPELL: u32 = 0x0048_d210;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors: `TESForm` (`01183028`), `SpellItem` (`01183060`),
/// `TESLevSpell` (`01187818`), `BaseFormComponent` (`01183040`),
/// `TESSpellList` (`01186ba4`), `TESActorBase` (`011846e8`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_SPELL_ITEM: u32 = 0x0118_3060;
const TYPE_TES_LEV_SPELL: u32 = 0x0118_7818;
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_TES_SPELL_LIST: u32 = 0x0118_6ba4;
const TYPE_TES_ACTOR_BASE: u32 = 0x0118_46e8;
/// `TESForm::GetFile` (Xbox PDB, `00484e60`, thiscall, `RET 4`).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB, `00485d50`, cdecl: id pointer, file).
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (cdecl, one word): the form with the form id given.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `005b5e40` (cdecl, variadic): prints a formatted warning.
const PRINT_WARNING: u32 = 0x005b_5e40;
/// "MASTERFILE: Unable to find Actor Effect (%08X) for unknown owner."
const TEXT_NO_OWNER: u32 = 0x0101_d648;
/// "MASTERFILE: Unable to find Actor Effect (%08X) for owner object (%08X)."
const TEXT_OWNER_ID: u32 = 0x0101_d690;
/// "MASTERFILE: Unable to find Actor Effect (%08X) for owner object \"%s\"."
const TEXT_OWNER_NAME: u32 = 0x0101_d6d8;

/// Removes the list's items one at a time while the node at `node` holds one
/// (the exe tests `node` for null again before each step; it does not change).
fn empty_list(e: &mut Engine, node: u32) {
    if node == 0 {
        return;
    }
    loop {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        e.call(NODE_REMOVE, &args![node]);
    }
}

// Translated from 0048d0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSpellList::FreeSpellList` (Xbox PDB): removes every spell from the
/// spell list (`this + 4`, through `00717e50`) and every leveled spell from
/// the list at `this + 0xC` (`0048d150`), one `0063f7b0` per item.
pub fn tes_spell_list_free_spell_list(e: &mut Engine, this: Ptr) {
    let spells = e.call(SPELL_LIST, &args![this]).u32();
    empty_list(e, spells);
    let leveled = fn_0048d150(e, this).addr();
    empty_list(e, leveled);
}

// Translated from 0048d170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSpellList::AddSpell` (Xbox PDB, `RET 4`: the spell): false for a null
/// spell or one the list already holds (`005f65d0`); otherwise adds it
/// (`005ae3d0`) and, when this list is the player's (`this` is the address
/// `004181e0` gives for the player plus 0x7C) and the `TES` flag `004516b0` is
/// clear, tells the player (`PlayerCharacter::SpellLearned`). Then true.
pub fn tes_spell_list_add_spell(e: &mut Engine, this: Ptr, spell: u32) -> bool {
    if spell == 0 {
        return false;
    }
    // The spell parameter's own stack word is what the list calls get and
    // what the player is told about afterwards.
    let added = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), spell);
        let list = e.call(SPELL_LIST, &args![this]).u32();
        if e.call(LIST_CONTAINS, &args![list, slot]).bool() {
            return None;
        }
        let list = e.call(SPELL_LIST, &args![this]).u32();
        e.call(LIST_ADD, &args![list, slot]);
        Some(e.mem.u32(slot.addr()))
    });
    let Some(spell) = added else {
        return false;
    };
    let player = e.global::<u32>(PLAYER);
    let base = e.call(PLAYER_BASE_FORM, &args![player]).u32();
    let players_list = if base != 0 { base + 0x7c } else { 0 };
    if this.addr() == players_list {
        let tes = e.global::<u32>(TES);
        if !e.call(TES_FLAG, &args![tes]).bool() {
            let player = e.global::<u32>(PLAYER);
            e.call(PLAYER_SPELL_LEARNED, &args![player, spell]);
        }
    }
    true
}

// Translated from 0048d290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSpellList::AddSpell_ov3` (Xbox PDB, `RET 4`: a form): when the `TES`
/// flag `004516b0` is set, `AddSpell` with the form as it is (its result
/// dropped) and true. Otherwise the form cast to `SpellItem` goes to
/// `AddSpell`, or, failing that, the form cast to `TESLevSpell` to
/// `AddSpell_ov2` (`0048d210`); false when it is neither.
pub fn tes_spell_list_add_spell_ov3(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let tes = e.global::<u32>(TES);
    if e.call(TES_FLAG, &args![tes]).bool() {
        tes_spell_list_add_spell(e, this, form);
        return true;
    }
    let spell = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_SPELL_ITEM, 0u32],
        )
        .u32();
    let leveled = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_LEV_SPELL, 0u32],
        )
        .u32();
    if spell != 0 {
        tes_spell_list_add_spell(e, this, spell)
    } else if leveled != 0 {
        e.call(ADD_LEVELED_SPELL, &args![this, leveled]).bool()
    } else {
        false
    }
}

/// The step `InitItem` takes on a node whose form is missing or not a spell:
/// the previous node's `00905330` (with the form id) and its next node when
/// this is the last node and a previous node exists; otherwise the node's
/// item is removed in place. Returns the node to look at next.
fn drop_node(e: &mut Engine, node: u32, previous: u32, id: u32) -> u32 {
    if e.call(NODE_NEXT, &args![node]).u32() == 0 && previous != 0 {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), id);
            e.call(NODE_REMOVE_AFTER, &args![previous, slot]);
        });
        e.call(NODE_NEXT, &args![previous]).u32()
    } else {
        e.call(NODE_REMOVE, &args![node]);
        node
    }
}

// Translated from 0048d400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSpellList::InitItem` (Xbox PDB, `RET 4`: the owning form or null):
/// turns the spell list's form ids into forms. For each node while it holds
/// an id: the id gets the owner's file compile index (`GetFile(-1)`; file 0
/// without an owner) and is looked up (`004839c0`).
/// - Not found: the node is dropped (see `drop_node`) and a `MASTERFILE`
///   warning printed (`005b5e40`): with the owner's name (vtable slot 0x130)
///   when `00474cb0` says the owner has one, with the owner's id (`0084e3a0`)
///   otherwise, or for an unknown owner.
/// - A `SpellItem`: stored in the node (`00726c60`); when this list belongs
///   to an actor base that is the player's (`004181e0`), the player learns
///   it. The node becomes the previous node; the next node follows.
/// - Otherwise: the node is dropped and a `TESLevSpell` is added through
///   `AddSpell_ov2`.
pub fn tes_spell_list_init_item(e: &mut Engine, this: Ptr, owner: u32) {
    let mut previous = 0u32;
    let mut node = e.call(SPELL_LIST, &args![this]).u32();
    let file = if owner != 0 {
        e.call(FORM_GET_FILE, &args![owner, 0xffff_ffffu32]).u32()
    } else {
        0
    };
    while node != 0 {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let (id, form) = e.with_stack(4, |e, slot| {
            let id = e.mem.u32(item);
            e.mem.set_u32(slot.addr(), id);
            e.call(FORM_ADD_COMPILE_INDEX, &args![slot, file]);
            let id = e.mem.u32(slot.addr());
            (id, e.call(LOOKUP_FORM, &args![id]).u32())
        });
        if form == 0 {
            node = drop_node(e, node, previous, id);
            if owner != 0 {
                if e.call(0x0047_4cb0, &args![owner]).u32() != 0 {
                    let name = e.vcall(owner, 0x130, &[]).u32();
                    e.call(PRINT_WARNING, &args![TEXT_OWNER_NAME, id, name]);
                } else {
                    let owner_id = e.call(0x0084_e3a0, &args![owner]).u32();
                    e.call(PRINT_WARNING, &args![TEXT_OWNER_ID, id, owner_id]);
                }
            } else {
                e.call(PRINT_WARNING, &args![TEXT_NO_OWNER, id]);
            }
            continue;
        }
        let spell = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_SPELL_ITEM, 0u32],
            )
            .u32();
        let leveled = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_LEV_SPELL, 0u32],
            )
            .u32();
        if spell != 0 {
            // The cast result's stack word goes to the node and, read again,
            // to the player.
            let spell = e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), spell);
                e.call(NODE_SET_ITEM, &args![node, slot]);
                e.mem.u32(slot.addr())
            });
            let actor_base = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![this, 0u32, TYPE_TES_SPELL_LIST, TYPE_TES_ACTOR_BASE, 0u32],
                )
                .u32();
            if actor_base != 0 {
                let player = e.global::<u32>(PLAYER);
                if actor_base == e.call(PLAYER_BASE_FORM, &args![player]).u32() {
                    let player = e.global::<u32>(PLAYER);
                    e.call(PLAYER_SPELL_LEARNED, &args![player, spell]);
                }
            }
            previous = node;
            node = e.call(NODE_NEXT, &args![node]).u32();
        } else {
            node = drop_node(e, node, previous, id);
            if leveled != 0 {
                e.call(ADD_LEVELED_SPELL, &args![this, leveled]);
            }
        }
    }
}

/// Adds every nonzero item of the list starting at `from` to the list `to`.
fn copy_items(e: &mut Engine, mut from: u32, to: u32) {
    while from != 0 {
        let item = e.call(NODE_ITEM, &args![from]).u32();
        let value = e.mem.u32(item);
        if value != 0 {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), value);
                e.call(LIST_ADD, &args![to, slot]);
            });
        }
        from = e.call(NODE_NEXT, &args![from]).u32();
    }
}

// Translated from 0048d630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSpellList::CopyComponent` (Xbox PDB, `RET 4`: the other component):
/// when the other component is a `TESSpellList` (`__RTDynamicCast` from
/// `BaseFormComponent`), empties this one (`FreeSpellList`) and adds the
/// other's spells, then its leveled spells, in order (`005ae3d0`).
pub fn tes_spell_list_copy_component(e: &mut Engine, this: Ptr, other: u32) {
    let source = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_SPELL_LIST,
                0u32
            ],
        )
        .u32();
    if source == 0 {
        return;
    }
    tes_spell_list_free_spell_list(e, this);
    let from = e.call(SPELL_LIST, &args![source]).u32();
    let to = e.call(SPELL_LIST, &args![this]).u32();
    copy_items(e, from, to);
    let from = fn_0048d150(e, Ptr::new(source)).addr();
    let to = fn_0048d150(e, this).addr();
    copy_items(e, from, to);
}

/// Writes the form ids of the nonzero items of the list starting at `node`
/// as one variable-sized value of the save buffer.
fn save_list(e: &mut Engine, buffer: u32, mut node: u32) {
    let mut count = 0u32;
    let start = e.call(0x0086_5f20, &args![buffer]).u32();
    while node != 0 {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let value = e.mem.u32(item);
        if value != 0 {
            e.call(0x0086_5df0, &args![buffer, value, 0u32]);
            count += 1;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(0x0086_5ff0, &args![buffer, count, start]);
}

// Translated from 0048e040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the spell lists to the save buffer (`RET 4`: the buffer) when its
/// flag 0x10 is set (`00428110` copies the buffer's word at +0x17 to a local,
/// `004280f0` tests the bit): the spells (`this + 4`), then the leveled spells
/// (`this + 0xC`), each as a count-prefixed run of form ids
/// (`BGSSaveGameBuffer::StartVariableSizedValue`, `SaveFormID_ov2` with 0,
/// `SaveVariableSizedValue_ov2`).
pub fn fn_0048e040(e: &mut Engine, this: Ptr, buffer: u32) {
    let saves = e.with_stack(4, |e, local| {
        let flags = e.call(0x0042_8110, &args![buffer, local]).u32();
        e.call(0x0042_80f0, &args![flags, 0x10u32]).bool()
    });
    if !saves {
        return;
    }
    save_list(e, buffer, this.addr() + 4);
    save_list(e, buffer, this.addr() + 0xc);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_d0d0, tes_spell_list_clear_data_component(Ptr)),
        entry!(0x0048d150, fn_0048d150(Ptr) -> Ptr),
        entry!(0x0048_d0f0, tes_spell_list_free_spell_list(Ptr)),
        entry!(0x0048_d170, tes_spell_list_add_spell(Ptr, u32) -> bool),
        entry!(0x0048_d290, tes_spell_list_add_spell_ov3(Ptr, u32) -> bool),
        entry!(0x0048_d400, tes_spell_list_init_item(Ptr, u32)),
        entry!(0x0048_d630, tes_spell_list_copy_component(Ptr, u32)),
        entry!(0x0048_e040, fn_0048e040(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_member_at_c() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x0048_d150, &args![0x0000_5000u32]).u32(),
            0x0000_500c
        );
    }
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
    fn fwd_a03_0048d0d0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0048_d0f0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_d0d0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0048_d0f0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod b024_tests {
    use crate::prelude::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ok(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// List doubles: a node is two words (item, next).
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.register(0x0071_7e50, |_, a| ret(a[0] + 4));
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x0072_6070, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0063_f7b0, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next != 0 {
                let (item, after) = (e.mem.u32(next), e.mem.u32(next + 4));
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            } else {
                e.mem.set_u32(a[0], 0);
            }
            Ret::default()
        });
        e.register(0x005f_65d0, |e, a| {
            let want = e.mem.u32(a[1]);
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) == want {
                    return ret(1);
                }
                node = e.mem.u32(node + 4);
            }
            ret(0)
        });
        e.register(0x005a_e3d0, |e, a| {
            let item = e.mem.u32(a[1]);
            if e.mem.u32(a[0]) == 0 {
                e.mem.set_u32(a[0], item);
                return Ret::default();
            }
            let mut node = a[0];
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            let new = e.mem.alloc(8);
            e.mem.set_u32(new, item);
            e.mem.set_u32(node + 4, new);
            Ret::default()
        });
        e.register(0x0090_5330, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(0x0072_6c60, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        e
    }

    /// Fills the inline list head at `head` with `items` (extra nodes allocated).
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

    fn items(e: &Engine, head: u32) -> Vec<u32> {
        let mut out = vec![];
        let mut node = head;
        while node != 0 && e.mem.u32(node) != 0 {
            out.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        out
    }

    #[test]
    fn at_0048d0f0_empties_both_lists() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x14);
        fill(&mut e, this + 4, &[1, 2, 3]);
        fill(&mut e, this + 0xc, &[4, 5]);
        e.call_log = Some(vec![]);
        e.call(0x0048_d0f0, &[this]);
        let log = e.call_log.take().unwrap();
        assert!(items(&e, this + 4).is_empty());
        assert!(items(&e, this + 0xc).is_empty());
        assert_eq!(only(&log, 0x0063_f7b0).len(), 5);
    }

    fn player_engine(player_list_owner: bool, tes_flag: u32) -> (Engine, u32) {
        let mut e = list_engine();
        let this = e.mem.alloc(0x14);
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global::<u32>(0x011d_ea3c, 0x0055_0000);
        e.set_global::<u32>(0x011c_3f2c, 0x0066_0000);
        let base = if player_list_owner {
            this - 0x7c
        } else {
            0x0077_0000
        };
        e.register_double(0x0041_81e0, move |_, _| ret(base));
        e.register_double(0x0045_16b0, move |_, _| ret(tes_flag));
        e.register(0x0094_c6c0, ok);
        (e, this)
    }

    #[test]
    fn at_0048d170_adds_new_spells_and_tells_the_player() {
        let (mut e, this) = player_engine(true, 0);
        fill(&mut e, this + 4, &[7]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0048_d170, &[this, 0]).bool());
        assert!(!e.call(0x0048_d170, &[this, 7]).bool());
        assert!(e.call(0x0048_d170, &[this, 9]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(items(&e, this + 4), vec![7, 9]);
        assert_eq!(only(&log, 0x0094_c6c0), vec![vec![0x0055_0000, 9]]);

        // The TES flag set: added, not told.
        let (mut e, this) = player_engine(true, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_d170, &[this, 9]).bool());
        assert!(only(&e.call_log.take().unwrap(), 0x0094_c6c0).is_empty());

        // Not the player's list.
        let (mut e, this) = player_engine(false, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_d170, &[this, 9]).bool());
        assert!(only(&e.call_log.take().unwrap(), 0x0094_c6c0).is_empty());
        assert_eq!(items(&e, this + 4), vec![9]);

        // A player with no base form: the list compared with is 0.
        let (mut e, this) = player_engine(false, 0);
        e.register(0x0041_81e0, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_d170, &[this, 9]).bool());
        assert!(only(&e.call_log.take().unwrap(), 0x0094_c6c0).is_empty());
    }

    /// Casts by a table: form 0x100 is a spell (0x101), 0x200 a leveled
    /// spell (0x201); the spell list 0 is cast to an actor base `base`.
    fn casts(e: &mut Engine) {
        e.register(0x00ec_43fb, |_, a| {
            ret(match (a[0], a[3]) {
                (0x100, 0x0118_3060) => 0x101,
                (0x200, 0x0118_7818) => 0x201,
                (0x300, 0x0118_46e8) => 0x301,
                _ => 0,
            })
        });
    }

    #[test]
    fn at_0048d290_casts_the_form() {
        let (mut e, this) = player_engine(false, 0);
        casts(&mut e);
        e.register(0x0048_d210, |_, _| ret(0x55));
        assert!(e.call(0x0048_d290, &[this, 0x100]).bool());
        assert_eq!(items(&e, this + 4), vec![0x101]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_d290, &[this, 0x200]).bool());
        assert!(!e.call(0x0048_d290, &[this, 0x999]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x0048_d210), vec![vec![this, 0x201]]);
        assert_eq!(
            only(&log, 0x00ec_43fb)[0],
            vec![0x200, 0, 0x0118_3028, 0x0118_3060, 0]
        );
        // The spell was already there: AddSpell's false comes back.
        assert!(!e.call(0x0048_d290, &[this, 0x100]).bool());

        // The TES flag: the form goes to AddSpell as it is; true.
        let (mut e, this) = player_engine(false, 1);
        casts(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_d290, &[this, 0x999]).bool());
        assert!(e.call(0x0048_d290, &[this, 0x999]).bool());
        assert!(only(&e.call_log.take().unwrap(), 0x00ec_43fb).is_empty());
        assert_eq!(items(&e, this + 4), vec![0x999]);
    }

    fn init_engine() -> (Engine, u32) {
        let (mut e, this) = player_engine(false, 0);
        e.register(0x0048_4e60, |_, _| ret(0x5000));
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        // Ids 0x0100_0001 (a spell), 0x0100_0002 (a leveled spell); others missing.
        e.register(0x0048_39c0, |_, a| {
            ret(match a[0] {
                0x0100_0001 => 0x100,
                0x0100_0002 => 0x200,
                _ => 0,
            })
        });
        casts(&mut e);
        e.register(0x0048_d210, ok);
        e.register(0x005b_5e40, ok);
        e.register(0x0047_4cb0, |_, a| ret(u32::from(a[0] == 0x0088_0000)));
        e.register(0x0084_e3a0, |_, _| ret(0x0bad));
        (e, this)
    }

    #[test]
    fn at_0048d400_resolves_the_spells() {
        let (mut e, this) = init_engine();
        // spell, missing, leveled, spell, missing (last).
        fill(&mut e, this + 4, &[1, 3, 2, 1, 4]);
        e.call_log = Some(vec![]);
        e.call(0x0048_d400, &[this, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(items(&e, this + 4), vec![0x101, 0x101]);
        assert!(only(&log, 0x0048_4e60).is_empty());
        assert_eq!(only(&log, 0x0048_5d50)[0][1], 0);
        assert_eq!(only(&log, 0x0048_d210), vec![vec![this, 0x201]]);
        // The middle missing one is removed in place, the last one through the
        // previous node.
        assert_eq!(only(&log, 0x0063_f7b0).len(), 2);
        assert_eq!(only(&log, 0x0090_5330).len(), 1);
        assert_eq!(
            only(&log, 0x005b_5e40),
            vec![
                vec![0x0101_d648, 0x0100_0003],
                vec![0x0101_d648, 0x0100_0004]
            ]
        );
        assert!(only(&log, 0x0094_c6c0).is_empty());
    }

    #[test]
    fn at_0048d400_warns_with_the_owner_and_tells_the_player() {
        // An owner with a name (slot 0x130) and one without.
        let (mut e, this) = init_engine();
        e.put_vtable(0x0090_0000, &[0x00a0_0000; 0x50]);
        e.mem.set_u32(0x0090_0130, 0x00a0_0130);
        e.register(0x00a0_0130, |_, _| ret(0x0012_3456));
        e.map(0x0088_0000, 0x1000);
        e.mem.set_u32(0x0088_0000, 0x0090_0000);
        fill(&mut e, this + 4, &[5, 1]);
        e.call_log = Some(vec![]);
        e.call(0x0048_d400, &[this, 0x0088_0000]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, 0x0048_4e60),
            vec![vec![0x0088_0000, 0xffff_ffff]]
        );
        assert_eq!(only(&log, 0x0048_5d50)[0][1], 0x5000);
        assert_eq!(
            only(&log, 0x005b_5e40),
            vec![vec![0x0101_d6d8, 0x0100_0005, 0x0012_3456]]
        );
        assert_eq!(items(&e, this + 4), vec![0x101]);

        let (mut e, this) = init_engine();
        e.map(0x0089_0000, 0x1000);
        fill(&mut e, this + 4, &[6]);
        e.call_log = Some(vec![]);
        e.call(0x0048_d400, &[this, 0x0089_0000]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, 0x005b_5e40),
            vec![vec![0x0101_d690, 0x0100_0006, 0x0bad]]
        );

        // The list belongs to the player's actor base: the player learns it.
        let (mut e, _) = init_engine();
        let list = 0x300;
        e.map(0, 0x1000);
        e.register(0x0041_81e0, |_, _| ret(0x301));
        fill(&mut e, list + 4, &[1]);
        e.call_log = Some(vec![]);
        e.call(0x0048_d400, &[list, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x0094_c6c0), vec![vec![0x0055_0000, 0x101]]);
    }

    #[test]
    fn at_0048d630_copies_both_lists() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x14);
        let other = e.mem.alloc(0x14);
        fill(&mut e, this + 4, &[1, 2]);
        fill(&mut e, other + 4, &[5, 0, 6]);
        fill(&mut e, other + 0xc, &[7]);
        e.register_double(0x00ec_43fb, move |_, a| {
            ret(if a[0] == other { other } else { 0 })
        });
        e.call(0x0048_d630, &[this, 0x1234]);
        assert_eq!(items(&e, this + 4), vec![1, 2]);
        e.call_log = Some(vec![]);
        e.call(0x0048_d630, &[this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, 0x00ec_43fb),
            vec![vec![other, 0, 0x0118_3040, 0x0118_6ba4, 0]]
        );
        assert_eq!(items(&e, this + 4), vec![5, 6]);
        assert_eq!(items(&e, this + 0xc), vec![7]);
    }

    #[test]
    fn at_0048e040_saves_both_lists_when_flag_10_is_set() {
        for (flag, saves) in [(0x10u32, true), (0xef, false)] {
            let mut e = list_engine();
            let this = e.mem.alloc(0x14);
            fill(&mut e, this + 4, &[5, 0, 6]);
            fill(&mut e, this + 0xc, &[7]);
            e.register(0x0042_8110, |e, a| {
                e.mem.set_u32(a[1], 0x77);
                ret(a[1])
            });
            e.register_double(0x0042_80f0, move |e, a| {
                assert_eq!(e.mem.u32(a[0]), 0x77);
                ret(u32::from(a[1] & flag != 0))
            });
            let mut start = 0;
            e.register_double(0x0086_5f20, move |_, _| {
                start += 0x10;
                ret(start)
            });
            e.register(0x0086_5df0, ok);
            e.register(0x0086_5ff0, ok);
            e.call_log = Some(vec![]);
            e.call(0x0048_e040, &[this, 0x4000]);
            let log = e.call_log.take().unwrap();
            assert_eq!(only(&log, 0x0042_80f0)[0][1], 0x10);
            if saves {
                assert_eq!(
                    only(&log, 0x0086_5df0),
                    vec![vec![0x4000, 5, 0], vec![0x4000, 6, 0], vec![0x4000, 7, 0]]
                );
                assert_eq!(
                    only(&log, 0x0086_5ff0),
                    vec![vec![0x4000, 2, 0x10], vec![0x4000, 1, 0x20]]
                );
            } else {
                assert!(only(&log, 0x0086_5f20).is_empty());
            }
        }
    }
}
