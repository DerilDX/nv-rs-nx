//! `fallout shared/tesfaction.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `005a8080` (`ECX` = faction): the address of the rank list (+0x3C).
const RANK_LIST: u32 = 0x005a_8080;
/// `006815c0` (`ECX` = node): the address of the node's item.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`ECX` = node): the next node, or 0.
const NODE_NEXT: u32 = 0x0072_6070;
/// `008256d0` (`ECX` = node): true when the node holds no item and has no
/// next node (an empty list).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `00905820` (`ECX` = node, the address of an item): adds the item after
/// the node.
const LIST_ADD: u32 = 0x0090_5820;
/// `0084e3a0` (`ECX` = form): the form id.
const FORM_ID: u32 = 0x0084_e3a0;
/// The form's name virtual slot.
const NAME_SLOT: u32 = 0x130;
/// The `TESDataHandler` singleton pointer.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `"DEFAULT: (Faction Reaction Error) Faction '%s' (%08X) is a Friend or
/// Ally of Faction '%s' (%08X), but Faction '%s' (%08X) is not ..."`.
const REACTION_ERROR: u32 = 0x0104_9a18;

// Translated from 005fd3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The faction's modifier toward `form`: `0048bf50` on the
/// `TESReactionForm` at +0x24.
pub fn fn_005fd3b0(e: &mut Engine, this: Ptr, form: u32) -> u32 {
    e.call(0x0048_bf50, &args![this.addr() + 0x24, form]).u32()
}

// Translated from 005fd430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the faction's reaction type toward `form` (`0048c220` on the
/// `TESReactionForm` at +0x24 with `reaction`), then vtable slot 0x48
/// (`AddChange` in the Xbox PDB's `TESForm` vtable) with 4.
pub fn fn_005fd430(e: &mut Engine, this: Ptr, form: u32, reaction: u32) {
    e.call(0x0048_c220, &args![this.addr() + 0x24, form, reaction]);
    e.vcall(this.addr(), 0x48, &args![4u32]);
}

// Translated from 005fcf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFaction::InitItem` (Xbox PDB): unless form flag 0x8 is set
/// (`004013e0`), resolves the reactions (`TESReactionForm::InitItem`,
/// `0048bc30`, with this faction as owner), sets form flag 0x8
/// (`00484ab0`) and, with a reputation form id at +0x38, gives it the
/// compile index of the faction's file and stores
/// `TESDataHandler::GetReputation` (`00461630`, on the handler at
/// `011c3f2c`) for it.
pub fn tes_faction_init_item(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    e.call(0x0048_bc30, &args![base + 0x24, this]);
    e.call(0x0048_4ab0, &args![this, true]);
    if e.mem.u32(base + 0x38) == 0 {
        return;
    }
    let id = e.with_stack(4, |e, id_at| {
        let id = e.mem.u32(base + 0x38);
        e.mem.set_u32(id_at.addr(), id);
        let file = e.call(0x0048_4e60, &args![this, u32::MAX]).u32();
        e.call(0x0048_5d50, &args![id_at, file]);
        e.mem.u32(id_at.addr())
    });
    let handler: u32 = e.global(DATA_HANDLER);
    let reputation = e.call(0x0046_1630, &args![handler, id]).u32();
    e.mem.set_u32(base + 0x38, reputation);
}

// Translated from 005fd460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fd460 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// Adds a new rank at the end of the faction's rank list and returns how
/// many ranks there were before: a new 0x1C-byte rank (`00401000`,
/// constructed by `005fcf10`, null when the allocation fails); the list
/// is walked while the node is not null and not empty (`008256d0`),
/// counting; the rank goes after the last node walked (`00905820`), or,
/// with none, into the list through `005ae3d0` (both with the address of a
/// stack copy of the rank).
pub fn fn_005fd460(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(0x0040_1000, &args![0x1cu32]).u32();
    let rank = if block != 0 {
        e.call(0x005f_cf10, &args![block]).u32()
    } else {
        0
    };
    let mut count = 0u32;
    let mut node = e.call(RANK_LIST, &args![this]).u32();
    let mut last = 0u32;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        count = count.wrapping_add(1);
        last = node;
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.with_stack(4, |e, rank_at| {
        e.mem.set_u32(rank_at.addr(), rank);
        if last != 0 {
            e.call(LIST_ADD, &args![last, rank_at]);
        } else {
            let list = e.call(RANK_LIST, &args![this]).u32();
            e.call(0x005a_e3d0, &args![list, rank_at]);
        }
    });
    count
}

// Translated from 005fd690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the faction into the save-game buffer `buffer`: the form's own
/// part (`00484d60`), the reactions (`0048c9d0` on +0x24), the 4-byte
/// `FACTION_DATA` (+0x34) when bit 0x2 of the buffer's flags word is set,
/// and the two crime words (+0x44, +0x48) when bit 0x80000000 is set (each
/// through `00865e50` with 4 bytes and 0; the flags word is copied to a
/// stack word by `00428110` and tested by `004280f0`).
pub fn fn_005fd690(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    e.call(0x0048_4d60, &args![this, buffer]);
    e.call(0x0048_c9d0, &args![base + 0x24, buffer]);
    let flag_set = |e: &mut Engine, mask: u32| {
        e.with_stack(4, |e, flags_at| {
            let flags = e.call(0x0042_8110, &args![buffer, flags_at]).u32();
            e.call(0x0042_80f0, &args![flags, mask]).bool()
        })
    };
    if flag_set(e, 2) {
        e.call(0x0086_5e50, &args![buffer, base + 0x34, 4u32, 0u32]);
    }
    if flag_set(e, 0x8000_0000) {
        e.call(0x0086_5e50, &args![buffer, base + 0x44, 4u32, 0u32]);
        e.call(0x0086_5e50, &args![buffer, base + 0x48, 4u32, 0u32]);
    }
}

// Translated from 005fd850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFaction::CheckFactionReactions` (Xbox PDB, `cdecl`, no arguments):
/// for every faction of the data handler's list (`00871a30` on the handler
/// at `011c3f2c`) and every reaction of it whose form is a faction (form
/// type 8, `00401170`) with reaction type 3 or 2, checks that the other
/// faction's reaction type toward this one (`0048c1b0`) is 2 or 3; if not,
/// prints the "Faction Reaction Error" message (twelve names and form ids,
/// fetched in the exe's order) and makes the other faction's reaction type
/// 3 (`005fd430`).
pub fn tes_faction_check_faction_reactions(e: &mut Engine) {
    let handler: u32 = e.global(DATA_HANDLER);
    let mut node = e.call(0x0087_1a30, &args![handler]).u32();
    while node != 0 {
        let at = e.call(NODE_ITEM, &args![node]).u32();
        let faction = e.mem.u32(at);
        if faction != 0 {
            let mut reaction_node = e.call(0x0071_7e50, &args![faction + 0x24]).u32();
            while reaction_node != 0 {
                let at = e.call(NODE_ITEM, &args![reaction_node]).u32();
                let reaction = e.mem.u32(at);
                if reaction != 0 {
                    check_reaction(e, faction, reaction);
                }
                reaction_node = e.call(NODE_NEXT, &args![reaction_node]).u32();
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

/// One reaction of `CheckFactionReactions`.
fn check_reaction(e: &mut Engine, faction: u32, reaction: u32) {
    let other = e.mem.u32(reaction);
    if other == 0 || e.call(0x0040_1170, &args![other]).u32() != 8 {
        return;
    }
    let kind = e.mem.u32(reaction + 8);
    if kind != 3 && kind != 2 {
        return;
    }
    let back = e.call(0x0048_c1b0, &args![other + 0x24, faction]).u32();
    if back == 2 || back == 3 {
        return;
    }
    let id = |e: &mut Engine, form: u32| e.call(FORM_ID, &args![form]).u32();
    let name = |e: &mut Engine, form: u32| e.vcall(form, NAME_SLOT, &[]).u32();
    // The words in push order (last argument first).
    let pushed = [
        id(e, faction),
        name(e, faction),
        name(e, faction),
        name(e, other),
        id(e, faction),
        name(e, faction),
        id(e, other),
        name(e, other),
        id(e, other),
        name(e, other),
        id(e, faction),
        name(e, faction),
    ];
    let mut words = vec![REACTION_ERROR];
    words.extend(pushed.iter().rev());
    e.call(0x005b_5e40, &words);
    fn_005fd430(e, Ptr::new(other), faction, 3);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_d3b0, fn_005fd3b0(Ptr, u32) -> u32),
        entry!(0x005f_d430, fn_005fd430(Ptr, u32, u32)),
        entry!(0x005f_cf80, tes_faction_init_item(Ptr)),
        entry!(0x005f_d460, fn_005fd460(Ptr) -> u32),
        entry!(0x005f_d690, fn_005fd690(Ptr, Ptr)),
        entry!(0x005f_d850, tes_faction_check_faction_reactions()),
    ]
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

    fn calls(e: &mut Engine, skip: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != skip)
            .collect()
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

    fn nop(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    #[test]
    fn b020_005fd3b0_asks_the_reaction_form() {
        let mut e = Engine::new();
        e.register(0x0048_bf50, |_, a| ret(a[0] + a[1]));
        assert_eq!(e.call(0x005f_d3b0, &args![0x1000u32, 5u32]).u32(), 0x1029);
    }

    #[test]
    fn b020_005fd430_sets_and_marks_changed() {
        let mut e = Engine::new();
        nop(&mut e, &[0x0048_c220, 0x0000_1048]);
        e.put_vtable(0x0150_0000, &[0; 0x20]);
        e.mem.set_u32(0x0150_0048, 0x0000_1048);
        let p = e.mem.alloc(0x4c);
        e.mem.set_u32(p, 0x0150_0000);
        e.call_log = Some(vec![]);
        e.call(0x005f_d430, &args![p, 0x77u32, 3u32]);
        assert_eq!(
            calls(&mut e, 0x005f_d430),
            vec![
                (0x0048_c220, vec![p + 0x24, 0x77, 3]),
                (0x0000_1048, vec![p, 4])
            ]
        );
    }

    #[test]
    fn b020_005fcf80_resolves_reactions_and_reputation() {
        let mut e = Engine::new();
        e.register(0x0040_13e0, |_, _| ret(0));
        nop(&mut e, &[0x0048_bc30, 0x0048_4ab0]);
        e.register(0x0048_4e60, |_, _| ret(0x77));
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(0x0046_1630, |_, a| ret(a[1] + a[0]));
        e.map(DATA_HANDLER, 4);
        e.set_global::<u32>(DATA_HANDLER, 0x10);
        let p = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        e.call(0x005f_cf80, &args![p]);
        assert_eq!(
            calls(&mut e, 0x005f_cf80),
            vec![
                (0x0040_13e0, vec![p]),
                (0x0048_bc30, vec![p + 0x24, p]),
                (0x0048_4ab0, vec![p, 1]),
            ]
        );
        e.mem.set_u32(p + 0x38, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x005f_cf80, &args![p]);
        assert_eq!(calls_to(&e, 0x0048_4e60), vec![vec![p, u32::MAX]]);
        assert_eq!(calls_to(&e, 0x0046_1630), vec![vec![0x10, 0x0100_0042]]);
        assert_eq!(e.mem.u32(p + 0x38), 0x0100_0052);
        e.register(0x0040_13e0, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x005f_cf80, &args![p]);
        assert_eq!(calls(&mut e, 0x005f_cf80).len(), 1);
    }

    /// Nodes are (item, next) pairs.
    fn chain_engine() -> Engine {
        let mut e = Engine::new();
        e.register(RANK_LIST, |_, a| ret(a[0] + 0x3c));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_IS_EMPTY, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
        nop(&mut e, &[LIST_ADD, 0x005a_e3d0]);
        e
    }

    #[test]
    fn b020_005fd460_appends_a_rank() {
        let mut e = chain_engine();
        e.register(0x0040_1000, |_, _| ret(0x0160_0000));
        e.register(0x005f_cf10, |_, a| ret(a[0]));
        let p = e.mem.alloc(0x4c);
        // An empty list: the rank goes in through 005ae3d0.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005f_d460, &args![p]).u32(), 0);
        let inserted = calls_to(&e, 0x005a_e3d0);
        assert_eq!(inserted.len(), 1);
        assert_eq!(inserted[0][0], p + 0x3c);
        assert_eq!(e.mem.u32(inserted[0][1]), 0x0160_0000);
        // Two ranks: after the second node.
        let second = e.mem.alloc(8);
        e.mem.set_u32(p + 0x3c, 0x11);
        e.mem.set_u32(p + 0x40, second);
        e.mem.set_u32(second, 0x22);
        e.register(0x0040_1000, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005f_d460, &args![p]).u32(), 2);
        let added = calls_to(&e, LIST_ADD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], second);
        assert_eq!(e.mem.u32(added[0][1]), 0);
        assert!(calls_to(&e, 0x005f_cf10).is_empty());
    }

    #[test]
    fn b020_005fd690_saves_by_flags() {
        let mut e = Engine::new();
        nop(&mut e, &[0x0048_4d60, 0x0048_c9d0, 0x0086_5e50]);
        e.register(0x0042_8110, |e, a| {
            let flags = e.mem.u32(a[0] + 0x17);
            e.mem.set_u32(a[1], flags);
            ret(a[1])
        });
        e.register(0x0042_80f0, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) & a[1] != 0))
        });
        let p = e.mem.alloc(0x4c);
        let buffer = e.mem.alloc(0x40);
        for (flags, saved) in [
            (0u32, vec![]),
            (2, vec![0x34]),
            (0x8000_0000, vec![0x44, 0x48]),
            (0x8000_0002, vec![0x34, 0x44, 0x48]),
        ] {
            e.mem.set_u32(buffer + 0x17, flags);
            e.call_log = Some(vec![]);
            e.call(0x005f_d690, &args![p, buffer]);
            assert_eq!(calls_to(&e, 0x0048_4d60), vec![vec![p, buffer]]);
            assert_eq!(calls_to(&e, 0x0048_c9d0), vec![vec![p + 0x24, buffer]]);
            let want: Vec<Vec<u32>> = saved.iter().map(|o| vec![buffer, p + o, 4, 0]).collect();
            assert_eq!(calls_to(&e, 0x0086_5e50), want);
        }
    }

    #[test]
    fn b020_005fd850_fixes_one_sided_alliances() {
        let mut e = Engine::new();
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0071_7e50, |_, a| ret(a[0] + 4));
        e.register(0x0040_1170, |e, a| ret(u32::from(e.mem.u8(a[0] + 4))));
        e.register(FORM_ID, |_, a| ret(a[0] | 0x0900_0000));
        e.register(0x0000_1130, |_, a| ret(a[0] | 0x0500_0000));
        nop(&mut e, &[0x005b_5e40, 0x0048_c220, 0x0000_1048]);
        e.put_vtable(0x0150_0000, &[0; 0x50]);
        e.mem.set_u32(0x0150_0130, 0x0000_1130);
        e.mem.set_u32(0x0150_0048, 0x0000_1048);
        // Factions a (ally of b, type 3) and b (reaction 0 toward a); c is
        // not a faction (type 9); d is an ally of a with an answer of 2.
        let faction = |e: &mut Engine, kind: u8| {
            let f = e.mem.alloc(0x4c);
            e.mem.set_u32(f, 0x0150_0000);
            e.mem.set_u8(f + 4, kind);
            f
        };
        let a = faction(&mut e, 8);
        let b = faction(&mut e, 8);
        let c = faction(&mut e, 9);
        let d = faction(&mut e, 8);
        let reaction = |e: &mut Engine, form: u32, kind: u32| {
            let r = e.mem.alloc(0xc);
            e.mem.set_u32(r, form);
            e.mem.set_u32(r + 8, kind);
            r
        };
        let r1 = reaction(&mut e, b, 3);
        let r2 = reaction(&mut e, c, 3);
        let r3 = reaction(&mut e, d, 2);
        // a's reaction list at a + 0x28: r1, r2, r3, then a null item.
        e.mem.set_u32(a + 0x28, r1);
        let n2 = e.mem.alloc(8);
        let n3 = e.mem.alloc(8);
        let n4 = e.mem.alloc(8);
        e.mem.set_u32(a + 0x2c, n2);
        e.mem.set_u32(n2, r2);
        e.mem.set_u32(n2 + 4, n3);
        e.mem.set_u32(n3, r3);
        e.mem.set_u32(n3 + 4, n4);
        e.register_double(0x0048_c1b0, move |_, args| {
            ret(if args[0] == d + 0x24 { 2 } else { 0 })
        });
        // The handler's faction list: a null item, then a.
        e.map(DATA_HANDLER, 4);
        let handler = e.mem.alloc(0x100);
        e.set_global::<u32>(DATA_HANDLER, handler);
        e.register(0x0087_1a30, |_, h| ret(h[0] + 0x88));
        let second = e.mem.alloc(8);
        e.mem.set_u32(handler + 0x8c, second);
        e.mem.set_u32(second, a);
        e.call_log = Some(vec![]);
        e.call(0x005f_d850, &[]);
        let printed = calls_to(&e, 0x005b_5e40);
        let na = a | 0x0500_0000;
        let ia = a | 0x0900_0000;
        let nb = b | 0x0500_0000;
        let ib = b | 0x0900_0000;
        assert_eq!(
            printed,
            vec![vec![
                REACTION_ERROR,
                na,
                ia,
                nb,
                ib,
                nb,
                ib,
                na,
                ia,
                nb,
                na,
                na,
                ia
            ]]
        );
        assert_eq!(calls_to(&e, 0x0048_c220), vec![vec![b + 0x24, a, 3]]);
        assert_eq!(calls_to(&e, 0x0000_1048), vec![vec![b, 4]]);
        assert_eq!(
            calls_to(&e, 0x0048_c1b0),
            vec![vec![b + 0x24, a], vec![d + 0x24, a]]
        );
    }
}
