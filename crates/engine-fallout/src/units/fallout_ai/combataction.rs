//! `fallout/ai/combat/combataction.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `CombatAction::CreateCombatAction` (the factory of the 27 action kinds),
//! the table of one action per kind at `011f1420` and the 20 lists at
//! `011f13d0` that index the actions by the first words of their +0x04
//! list's entries.

#[allow(unused_imports)]
use crate::prelude::*;

/// The action table (27 pointers, one per kind).
const ACTIONS: u32 = 0x011f_1420;
/// The lists (20 pointers) the actions are filed in.
const ACTION_LISTS: u32 = 0x011f_13d0;
/// Number of action kinds and of lists.
const ACTION_KINDS: u32 = 0x1b;
const LIST_COUNT: u32 = 0x14;
/// `operator new` (`00401000`, `cdecl(size)`) and `operator delete`
/// (`00401030`, `cdecl(block)`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The game's `memset` wrapper (`00403d30`, `cdecl(dst, value, count)`).
const MEMSET: u32 = 0x0040_3d30;
/// The debug print `005b5e40` (`cdecl(format, ...)`) and
/// `"COMBAT: Could not create combat action of type %i"` (`0108cacc`).
const DEBUG_PRINT: u32 = 0x005b_5e40;
const CREATE_FAILED_FORMAT: u32 = 0x0108_cacc;
/// The two-word list head constructor (`0096a2d0`, `thiscall`).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// Appends to a list (`00905820`, `thiscall(element pointer)`).
const LIST_APPEND: u32 = 0x0090_5820;
/// Empties a list (`00470470`, `thiscall`) and its scalar deleting
/// destructor (`004702f0`, `thiscall(flags)`).
const LIST_REMOVE_ALL: u32 = 0x0047_0470;
const LIST_DELETE: u32 = 0x0047_02f0;
/// The first node of an action +0x04 list (`007d6bb0`, `thiscall`), a
/// node item address (`006815c0`), the next node (`0062d990`) and an
/// entry first word (`00559450`).
const FIRST_NODE: u32 = 0x007d_6bb0;
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0062_d990;
const ENTRY_FIRST: u32 = 0x0055_9450;
/// The `CombatAction` destructor body (`00979c80`, `thiscall`).
const ACTION_DESTRUCT: u32 = 0x0097_9c80;
/// Every action object is 0x2C bytes.
const ACTION_SIZE: u32 = 0x2c;

/// The constructor of each kind (the jump table at `0097a924`, as arms).
fn action_constructor(kind: u32) -> Option<u32> {
    let constructor = match kind {
        0 => 0x0097_b0b0,  // CombatActionAttackRangedExplosive
        1 => 0x0097_b280,  // CombatActionAttackRangedExplosiveFromCover
        2 => 0x0097_ad10,  // CombatActionAttackRanged
        3 => 0x0097_aeb0,  // CombatActionAttackRangedFromCover
        4 => 0x0097_b620,  // CombatActionAttackGrenade
        5 => 0x0097_b7a0,  // CombatActionAttackGrenadeFlushTarget
        6 => 0x0097_b9e0,  // CombatActionAttackGrenadeFromCover
        7 => 0x0097_b350,  // CombatActionAttackMelee
        8 => 0x0097_b4e0,  // CombatActionAttackHandToHand
        9 => 0x0097_bbd0,  // CombatActionMove
        10 => 0x0097_bfe0, // CombatActionMoveAndSwitchToMelee
        11 => 0x0097_c170, // CombatActionMoveAndAttackRanged
        12 => 0x0097_c3a0, // CombatActionMoveAndAttackRangedExplosive
        13 => 0x0097_c560, // CombatActionMoveAndAttackGrenade
        14 => 0x0097_c700, // CombatActionDrawWeapon
        15 => 0x0097_c850, // CombatActionSwitchWeapon
        16 => 0x0097_c9d0, // CombatActionAvoidThreat
        17 => 0x0097_ca60, // CombatActionSearch
        18 => 0x0097_cb70, // CombatActionInvestigate
        19 => 0x0097_cc00, // CombatActionDodge
        20 => 0x0097_cc80, // CombatActionIgnoreBlockedTarget
        21 => 0x0097_cd30, // CombatActionFlee
        22 => 0x0097_ce90, // CombatActionActivateCombatItem
        23 => 0x0097_cfe0, // CombatActionUseCombatItem
        24 => 0x0097_d1c0, // CombatActionAcquireLineOfSight
        25 => 0x0097_d250, // CombatActionHide
        26 => 0x0097_d130, // CombatActionApproachTarget
        _ => return None,
    };
    Some(constructor)
}

// Translated from 00979f90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00979f90 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatAction::CreateCombatAction` (Xbox PDB; `cdecl(kind)`): for kinds
/// 0..=26 allocates 0x2C bytes and runs that kind constructor with `kind`
/// (the class names are the RTTI names of the vtables they set); returns the
/// action, or null when the allocation fails. Any other kind prints
/// `"COMBAT: Could not create combat action of type %i"` (`005b5e40`) and
/// returns null.
pub fn combat_action_create_combat_action(e: &mut Engine, kind: u32) -> Ptr {
    let Some(constructor) = action_constructor(kind) else {
        e.call(DEBUG_PRINT, &args![CREATE_FAILED_FORMAT, kind]);
        return Ptr::NULL;
    };
    let block = e.call(OPERATOR_NEW, &args![ACTION_SIZE]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(constructor, &args![block, kind]).ptr()
}

// Translated from 0097a990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no parameters): clears the 20 lists
/// at `011f13d0`, makes the action of each kind
/// ([`combat_action_create_combat_action`]) into the table at `011f1420`,
/// and files each action in the list of the first word of every entry of its
/// +0x04 list ([`fn_0097ab30`]).
pub fn fn_0097a990(e: &mut Engine) {
    e.call(MEMSET, &args![ACTION_LISTS, 0u32, 0x50u32]);
    for kind in 0..ACTION_KINDS {
        let action = combat_action_create_combat_action(e, kind).addr();
        e.set_global(ACTIONS + 4 * kind, action);
        if action == 0 {
            continue;
        }
        let mut node = e.call(FIRST_NODE, &args![action]).u32();
        while node != 0 {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            let entry = e.mem.u32(slot);
            let index = e.call(ENTRY_FIRST, &args![entry]).u32();
            fn_0097ab30(e, index, action);
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
}

// Translated from 0097aa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no parameters): deletes every
/// action of the table at `011f1420` ([`fn_0097ab00`] with 1), then empties
/// and deletes every list at `011f13d0` (`00470470`, `004702f0(1)`). The
/// table entries are left as they are.
pub fn fn_0097aa40(e: &mut Engine) {
    for kind in 0..ACTION_KINDS {
        let action: u32 = e.global(ACTIONS + 4 * kind);
        if action != 0 {
            fn_0097ab00(e, Ptr::new(action), 1);
        }
    }
    for index in 0..LIST_COUNT {
        let list: u32 = e.global(ACTION_LISTS + 4 * index);
        if list != 0 {
            e.call(LIST_REMOVE_ALL, &args![list]);
            let list: u32 = e.global(ACTION_LISTS + 4 * index);
            if list != 0 {
                e.call(LIST_DELETE, &args![list, 1u32]);
            }
        }
    }
}

// Translated from 0097ab00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatAction` scalar deleting destructor (unnamed in the engine map; by
/// its body): the destructor body (`00979c80`), then `operator delete` when
/// bit 0 of `flags` is set. Returns `this`.
pub fn fn_0097ab00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(ACTION_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0097ab30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097ab30 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Unnamed in the engine map (`cdecl(index, action)`): makes list `index`
/// of `011f13d0` when it is null (8 bytes, `0096a2d0`; null when the
/// allocation fails) and appends `action` to it (`00905820`, given the
/// address of the `action` argument).
pub fn fn_0097ab30(e: &mut Engine, index: u32, action: u32) {
    let entry = ACTION_LISTS.wrapping_add(index.wrapping_mul(4));
    if e.mem.u32(entry) == 0 {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let list = if block != 0 {
            e.call(LIST_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(entry, list);
    }
    let list = e.mem.u32(entry);
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), action);
        e.call(LIST_APPEND, &args![list, argument]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0097_9f90,
            combat_action_create_combat_action(u32) -> Ptr
        ),
        entry!(0x0097_a990, fn_0097a990()),
        entry!(0x0097_aa40, fn_0097aa40()),
        entry!(0x0097_ab00, fn_0097ab00(Ptr, u32) -> Ptr),
        entry!(0x0097_ab30, fn_0097ab30(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_1000, 0x1000);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        for kind in 0..ACTION_KINDS {
            let constructor = action_constructor(kind).unwrap();
            e.register(constructor, |e, a| {
                e.mem.set_u32(a[0] + 0x24, a[1]);
                a[0].into_ret()
            });
        }
        e.register(DEBUG_PRINT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn create_runs_the_kind_constructor() {
        let mut e = engine();
        for kind in 0..ACTION_KINDS {
            let action = e.call(0x0097_9f90, &args![kind]).u32();
            assert_ne!(action, 0);
            assert_eq!(e.mem.u32(action + 0x24), kind);
            let constructor = action_constructor(kind).unwrap();
            assert_eq!(calls(&e, constructor), vec![vec![action, kind]]);
        }
        assert_eq!(action_constructor(2), Some(0x0097_ad10));
        assert_eq!(action_constructor(26), Some(0x0097_d130));
        let sizes: Vec<u32> = calls(&e, OPERATOR_NEW).iter().map(|w| w[0]).collect();
        assert_eq!(sizes, vec![0x2c; 27]);
    }

    #[test]
    fn create_reports_an_unknown_kind_and_a_failed_allocation() {
        let mut e = engine();
        assert_eq!(e.call(0x0097_9f90, &args![27u32]).u32(), 0);
        assert_eq!(calls(&e, DEBUG_PRINT), vec![vec![CREATE_FAILED_FORMAT, 27]]);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0097_9f90, &args![4u32]).u32(), 0);
        assert!(calls(&e, 0x0097_b620).is_empty());
    }

    #[test]
    fn file_makes_the_list_once_and_appends() {
        let mut e = engine();
        e.register(LIST_CONSTRUCT, |_, a| (a[0] + 1).into_ret());
        let appended: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        let record = appended.clone();
        e.register_double(LIST_APPEND, move |e, a| {
            record.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call(0x0097_ab30, &args![3u32, 0x7000u32]);
        e.call(0x0097_ab30, &args![3u32, 0x7100u32]);
        let list = e.global::<u32>(ACTION_LISTS + 12);
        assert_eq!(calls(&e, LIST_CONSTRUCT), vec![vec![list - 1]]);
        assert_eq!(*appended.borrow(), vec![(list, 0x7000), (list, 0x7100)]);
    }

    #[test]
    fn set_up_makes_every_action_and_files_it() {
        let mut e = engine();
        e.register(MEMSET, |_, _| Ret::default());
        e.register(LIST_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(LIST_APPEND, |_, _| Ret::default());
        // Every action has one entry, whose first word is kind % 20.
        e.register(FIRST_NODE, |_, a| a[0].into_ret());
        e.register(NODE_ITEM, |_, a| (a[0] + 0x24).into_ret());
        e.register(ENTRY_FIRST, |_, a| (a[0] % 20).into_ret());
        e.register(NODE_NEXT, |_, _| 0u32.into_ret());
        e.call(0x0097_a990, &args![]);
        assert_eq!(calls(&e, MEMSET), vec![vec![ACTION_LISTS, 0, 0x50]]);
        // Kind 0 stores 0 at +0x24: its node has no entry.
        assert_eq!(calls(&e, ENTRY_FIRST).len(), 26);
        assert_eq!(calls(&e, LIST_APPEND).len(), 26);
        for kind in 0..ACTION_KINDS {
            assert_ne!(e.global::<u32>(ACTIONS + 4 * kind), 0);
        }
    }

    #[test]
    fn tear_down_deletes_actions_and_lists() {
        let mut e = engine();
        for addr in [
            ACTION_DESTRUCT,
            OPERATOR_DELETE,
            LIST_REMOVE_ALL,
            LIST_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.set_global(ACTIONS + 8, 0x7000u32);
        e.set_global(ACTION_LISTS + 4, 0x7100u32);
        e.call(0x0097_aa40, &args![]);
        assert_eq!(calls(&e, ACTION_DESTRUCT), vec![vec![0x7000]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x7000]]);
        assert_eq!(calls(&e, LIST_REMOVE_ALL), vec![vec![0x7100]]);
        assert_eq!(calls(&e, LIST_DELETE), vec![vec![0x7100, 1]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0097_ab00, &args![0x7000u32, 0u32]).u32(), 0x7000);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
    }
}
