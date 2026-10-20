//! `fallout/ai/run_009aa120` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The add and remove of the pair lists the `CombatAction` and `CombatGoal`
//! classes keep (8-byte entries built by `004b0680`, compared by their first
//! word).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00401000`, `cdecl(size)`) and `operator delete`
/// (`00401030`, `cdecl(block)`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The entry constructor (`004b0680`, `thiscall(a, b)`).
const ENTRY_CONSTRUCT: u32 = 0x004b_0680;
/// Appends an entry to the list (`00905820`, `thiscall(entry pointer)`).
const LIST_APPEND: u32 = 0x0090_5820;
/// Returns the address of a node item (`006815c0`, `thiscall`).
const NODE_ITEM: u32 = 0x0068_15c0;
/// Returns the first word of an entry (`00559450`, `thiscall`).
const ENTRY_FIRST: u32 = 0x0055_9450;
/// Removes an entry from the list (`00905330`, `thiscall(entry pointer)`).
const LIST_REMOVE: u32 = 0x0090_5330;
/// Returns the next node (`0062d990`, `thiscall`).
const NODE_NEXT: u32 = 0x0062_d990;

// Translated from 009aa2b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009aa2b0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Unnamed in the engine map (`thiscall(a, b)`, `RET 8`): makes an 8-byte
/// entry `(a, b)` (`004b0680`; null when the allocation fails) and appends
/// it to the list at `this` (`00905820`, given the address of a local
/// holding the entry).
pub fn fn_009aa2b0(e: &mut Engine, this: Ptr, a: u32, b: u32) {
    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let entry = if block != 0 {
        e.call(ENTRY_CONSTRUCT, &args![block, a, b]).u32()
    } else {
        0
    };
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), entry);
        e.call(LIST_APPEND, &args![this, slot]);
    });
}

// Translated from 009aa340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(value)`, `RET 4`): walks the list
/// from `this` (`0062d990` to the next node) and removes and frees the first
/// entry whose first word is `value` (`00905330` on the node holding it,
/// then `operator delete`). Nodes without an entry are skipped.
pub fn fn_009aa340(e: &mut Engine, this: Ptr, value: u32) {
    let mut node = this.addr();
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if entry != 0 && e.call(ENTRY_FIRST, &args![entry]).u32() == value {
            let entry = e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), entry);
                e.call(LIST_REMOVE, &args![node, local]);
                e.mem.u32(local.addr())
            });
            e.call(OPERATOR_DELETE, &args![entry]);
            return;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009a_a2b0, fn_009aa2b0(Ptr, u32, u32)),
        entry!(0x009a_a340, fn_009aa340(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn add_makes_an_entry_and_appends_it() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ENTRY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        let appended = e.mem.alloc(4);
        e.register_double(LIST_APPEND, move |e, a| {
            let entry = e.mem.u32(a[1]);
            e.mem.set_u32(appended, entry);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009a_a2b0, &args![0x5000u32, 3u32, 4u32]);
        let entry = e.mem.u32(appended);
        assert_eq!(calls(&e, ENTRY_CONSTRUCT), vec![vec![entry, 3, 4]]);
        assert_eq!(e.mem.u32(entry + 4), 4);
        assert_eq!(calls(&e, LIST_APPEND)[0][0], 0x5000);
        // A failed allocation appends a null entry.
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.call(0x009a_a2b0, &args![0x5000u32, 3u32, 4u32]);
        assert_eq!(e.mem.u32(appended), 0);
    }

    #[test]
    fn remove_frees_the_first_matching_entry() {
        let mut e = Engine::new();
        // Nodes: item word at +0, next at +4. Entries: first word at +0.
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ENTRY_FIRST, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LIST_REMOVE, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let entry = |e: &mut Engine, v: u32| {
            let p = e.mem.alloc(8);
            e.mem.set_u32(p, v);
            p
        };
        let a = entry(&mut e, 1);
        let b = entry(&mut e, 2);
        let c = entry(&mut e, 2);
        let mut next = 0;
        for item in [c, b, 0, a] {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        let head = next;
        let third = e.mem.u32(e.mem.u32(head + 4) + 4);
        e.call_log = Some(vec![]);
        e.call(0x009a_a340, &args![head, 2u32]);
        assert_eq!(calls(&e, LIST_REMOVE)[0][0], third);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![b]]);
        // No match: nothing removed.
        e.call_log = Some(vec![]);
        e.call(0x009a_a340, &args![head, 9u32]);
        assert!(calls(&e, LIST_REMOVE).is_empty());
        assert_eq!(calls(&e, NODE_NEXT).len(), 4);
    }
}
