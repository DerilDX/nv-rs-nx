//! `fallout shared/tescondition.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Only `TESCondition::IsTrue_ov2` (`00680c60`) so far: the evaluation of a
//! condition list with its AND/OR grouping.

#[allow(unused_imports)]
use crate::prelude::*;

/// The list head of the condition list (`006815c0`, `this`): also, on a
/// node, the address of the node's item word.
const LIST_NODE_ITEM_SLOT: u32 = 0x0068_15c0;
/// Whether a list node is the end of the list (`008256d0`, `ECX` = node;
/// the exe has no name for it).
const NODE_IS_END: u32 = 0x0082_56d0;
/// `005693d0(ECX = reference)`: a replacement for the reference argument
/// when it answers nonzero.
const REFERENCE_REPLACEMENT: u32 = 0x0056_93d0;
/// `00413f40(ECX = item)`: the address of the item's 16-bit function
/// number.
const ITEM_FUNCTION_NUMBER: u32 = 0x0041_3f40;
/// `005ec040(ECX = item)`: whether the item is joined to the next by OR.
const ITEM_IS_OR: u32 = 0x005e_c040;
/// `00681600(ECX = item, arg0, reference, &out flag)` (`RET 0xC`): the
/// result of one condition item.
const ITEM_EVALUATE: u32 = 0x0068_1600;
/// The next list node (`00726070`).
const NODE_NEXT: u32 = 0x0072_6070;

// Translated from 00680c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCondition::IsTrue_ov2` (Xbox PDB): evaluates the condition list at
/// `this` (`ECX`; `RET 0x10`, four stack words) and returns whether it
/// holds. `arg0` and `reference` are handed to each item (`reference` is
/// first replaced by `005693d0(reference)` when that answers nonzero);
/// `out_flag` is a byte that is set to 1 while a failing item reports its
/// own flag (or an OR group fails without an all-false item) and is
/// cleared at the end when the list holds or an item failed outright.
/// With `filter` set, items whose function number is below 0x44, or above
/// 0x48 (except 0x174), are not evaluated and count as true.
///
/// An item joined by OR starts a group that runs to the next item that is
/// not; the group is true when any item is. (The decompiler's output was
/// checked against the disassembly: the byte locals are `all_true` (-9),
/// `current` (-4), `in_or_group` (-0xa), `or_flag` (-1), `or_all_false`
/// (-3) and `failed` (-2).)
pub fn tes_condition_is_true_ov2(
    e: &mut Engine,
    this: Ptr,
    arg0: u32,
    reference: u32,
    out_flag: Ptr,
    filter: u8,
) -> bool {
    let mut node = e.call(LIST_NODE_ITEM_SLOT, &args![this]).u32();
    if e.call(NODE_IS_END, &args![node]).bool() {
        e.mem.set_u8(out_flag.addr(), 0);
        return true;
    }
    let mut all_true = true;
    let mut current = false;
    let mut in_or_group = false;
    let mut or_flag = false;
    let mut or_all_false = false;
    e.mem.set_u8(out_flag.addr(), 0);
    let mut failed = false;
    let mut reference = reference;
    if reference != 0 {
        let replacement = e.call(REFERENCE_REPLACEMENT, &args![reference]).u32();
        if replacement != 0 {
            reference = replacement;
        }
    }
    // The item's own flag byte: a local of the game's stack, passed by address.
    e.with_stack(4, |e, local| {
        while node != 0 && !e.call(NODE_IS_END, &args![node]).bool() {
            let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
            let item = e.mem.u32(slot);
            let mut skip = false;
            if filter != 0 {
                let function = e.call(ITEM_FUNCTION_NUMBER, &args![item]).u32();
                let function = e.mem.u16(function) as i32;
                if function < 0x44 || (function > 0x48 && function != 0x174) {
                    skip = true;
                }
            }
            e.mem.set_u8(local.addr(), 0);
            if !in_or_group {
                if !e.call(ITEM_IS_OR, &args![item]).bool() {
                    current = if skip {
                        true
                    } else {
                        e.call(ITEM_EVALUATE, &args![item, arg0, reference, local])
                            .bool()
                    };
                    all_true = all_true && current;
                    if e.mem.u8(local.addr()) != 0 {
                        e.mem.set_u8(out_flag.addr(), 1);
                    } else if !current {
                        failed = true;
                    }
                } else {
                    current = e
                        .call(ITEM_EVALUATE, &args![item, arg0, reference, local])
                        .bool();
                    in_or_group = true;
                    or_all_false = false;
                    or_flag = e.mem.u8(local.addr()) != 0;
                    if !or_flag && !current {
                        or_all_false = true;
                    }
                }
            } else {
                if !current {
                    current = if skip {
                        skip
                    } else {
                        e.call(ITEM_EVALUATE, &args![item, arg0, reference, local])
                            .bool()
                    };
                    or_flag = or_flag || e.mem.u8(local.addr()) != 0;
                    if e.mem.u8(local.addr()) == 0 && !current {
                        or_all_false = true;
                    }
                }
                if !e.call(ITEM_IS_OR, &args![item]).bool() {
                    in_or_group = false;
                    all_true = all_true && current;
                    if !all_true {
                        if or_flag && !or_all_false {
                            e.mem.set_u8(out_flag.addr(), 1);
                        } else {
                            failed = true;
                        }
                    }
                }
            }
            if !all_true && !in_or_group && e.mem.u8(out_flag.addr()) == 0 {
                break;
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
    if in_or_group {
        all_true = all_true && current;
        if !all_true {
            if or_flag && !or_all_false {
                e.mem.set_u8(out_flag.addr(), 1);
            } else {
                failed = true;
            }
        }
    }
    if all_true || failed {
        e.mem.set_u8(out_flag.addr(), 0);
    }
    all_true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00680c60, tes_condition_is_true_ov2(Ptr, u32, u32, Ptr, u8) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An item: +0 or-joined, +1 result, +2 own flag, +4 function number.
    fn item(e: &mut Engine, or: bool, result: bool, flag: bool, function: u16) -> u32 {
        let p = e.mem.alloc(8);
        e.mem.set_u8(p, or as u8);
        e.mem.set_u8(p + 1, result as u8);
        e.mem.set_u8(p + 2, flag as u8);
        e.mem.set_u16(p + 4, function);
        p
    }

    /// A list of nodes `{item, next}`; the head's address answers `this`.
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for &it in items.iter().rev() {
            let n = e.mem.alloc(8);
            e.mem.set_u32(n, it);
            e.mem.set_u32(n + 4, next);
            next = n;
        }
        next
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn setup() -> Engine {
        let mut e = Engine::new();
        // A node's item word is at the node itself; a null node is the end.
        e.register(LIST_NODE_ITEM_SLOT, |_, a| ret(a[0]));
        e.register(NODE_IS_END, |_, a| ret((a[0] == 0) as u32));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(REFERENCE_REPLACEMENT, |_, a| {
            ret(if a[0] == 0x100 { 0x200 } else { 0 })
        });
        e.register(ITEM_FUNCTION_NUMBER, |_, a| ret(a[0] + 4));
        e.register(ITEM_IS_OR, |e, a| ret(e.mem.u8(a[0]) as u32));
        e.register(ITEM_EVALUATE, |e, a| {
            e.mem.set_u8(a[3], e.mem.u8(a[0] + 2));
            ret(e.mem.u8(a[0] + 1) as u32)
        });
        e.call_log = Some(vec![]);
        e
    }

    fn run(e: &mut Engine, items: &[u32], filter: u8) -> (bool, u8) {
        let head = list(e, items);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 9);
        let ok = e
            .call(
                0x0068_0c60,
                &args![
                    Ptr::<()>::new(head),
                    5u32,
                    0x100u32,
                    Ptr::<()>::new(out),
                    filter
                ],
            )
            .bool();
        (ok, e.mem.u8(out))
    }

    fn evaluations(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ITEM_EVALUATE)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn an_empty_list_is_true_and_clears_the_flag() {
        let mut e = setup();
        assert_eq!(run(&mut e, &[], 0), (true, 0));
        assert!(evaluations(&e).is_empty());
    }

    #[test]
    fn and_items_stop_at_the_first_false_one() {
        let mut e = setup();
        let t = item(&mut e, false, true, false, 0x50);
        let f = item(&mut e, false, false, false, 0x50);
        let never = item(&mut e, false, true, false, 0x50);
        assert_eq!(run(&mut e, &[t, t], 0), (true, 0));
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, &[t, f, never], 0), (false, 0));
        let seen = evaluations(&e);
        assert_eq!(seen.len(), 2);
        // Each item gets the first argument and the replaced reference.
        assert_eq!(seen[0][1..3], [5, 0x200]);
    }

    #[test]
    fn a_failing_item_with_its_own_flag_keeps_the_out_flag_and_goes_on() {
        let mut e = setup();
        let flagged = item(&mut e, false, false, true, 0x50);
        let t = item(&mut e, false, true, false, 0x50);
        assert_eq!(run(&mut e, &[flagged, t], 0), (false, 1));
        assert_eq!(evaluations(&e).len(), 2);
    }

    #[test]
    fn an_or_group_is_true_when_any_item_is() {
        let mut e = setup();
        let a = item(&mut e, true, false, false, 0x50);
        let b = item(&mut e, false, true, false, 0x50);
        let f = item(&mut e, false, false, false, 0x50);
        let t = item(&mut e, false, true, false, 0x50);
        // false OR true, then AND true.
        assert_eq!(run(&mut e, &[a, b, t], 0), (true, 0));
        // true OR (not evaluated), then AND false.
        let a2 = item(&mut e, true, true, false, 0x50);
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, &[a2, b, f], 0), (false, 0));
        assert_eq!(evaluations(&e).len(), 2);
    }

    #[test]
    fn a_false_or_group_fails_and_a_flagged_one_reports_it() {
        let mut e = setup();
        let a = item(&mut e, true, false, false, 0x50);
        let b = item(&mut e, false, false, false, 0x50);
        assert_eq!(run(&mut e, &[a, b], 0), (false, 0));
        // A group left open at the end of the list closes after the loop.
        assert_eq!(run(&mut e, &[a], 0), (false, 0));
        // Every item of the group flagged and false: the flag is reported
        // only when no item was false without a flag.
        let af = item(&mut e, true, false, true, 0x50);
        let bf = item(&mut e, false, false, true, 0x50);
        assert_eq!(run(&mut e, &[af, bf], 0), (false, 1));
    }

    #[test]
    fn the_filter_skips_items_outside_the_function_range() {
        let mut e = setup();
        for (function, evaluated) in [
            (0x10u16, false),
            (0x43, false),
            (0x44, true),
            (0x48, true),
            (0x49, false),
            (0x174, true),
            (0x175, false),
        ] {
            let f = item(&mut e, false, false, false, function);
            e.call_log = Some(vec![]);
            let (ok, _) = run(&mut e, &[f], 1);
            assert_eq!(evaluations(&e).len(), evaluated as usize, "{function:#x}");
            assert_eq!(ok, !evaluated, "{function:#x}");
        }
        // Without the filter every item is evaluated.
        let f = item(&mut e, false, false, false, 0x10);
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, &[f], 0), (false, 0));
        assert_eq!(evaluations(&e).len(), 1);
    }
}
