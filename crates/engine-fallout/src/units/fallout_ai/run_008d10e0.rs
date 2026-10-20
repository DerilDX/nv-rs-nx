//! `fallout/ai/run_008d10e0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008d1160 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00c705c0` of unit bhkcharactercontroller (the
/// character controller's destructor body), and returns nothing. Parameters: one word,
/// `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX` on.
/// `00c705c0`'s translated signature `fn_00c705c0(Ptr<Ctl>)` returns nothing.
pub fn fn_008d1160(e: &mut Engine, this: Ptr) {
    e.call(0x00c7_05c0, &args![this]);
}

// Translated from 008d1550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x410` (`SUB ECX,0x410`) and jumps to `008d1130`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_008d1550(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x008d_1130, &args![this.addr().wrapping_sub(0x410), flags])
        .u32()
}

/// The process list the two functions below walk and change (a global
/// `BSSimpleList`-shaped list at `011dfe14`).
const PROCESS_LIST: u32 = 0x011d_fe14;
/// `008256d0` (`thiscall`, no stack words): whether the list is empty (`AL`).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0` (`thiscall`, no stack words): the list's first item address.
const LIST_ITEM: u32 = 0x0068_15c0;
/// `0063f7b0` (`thiscall`, no stack words): removes the list's head.
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// `00905330` (`thiscall`, `RET 4`): takes the address of an item word.
const LIST_ADD: u32 = 0x0090_5330;

// Translated from 008d1560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the list at `011dfe14` (no parameters: `ECX` is not read, plain
/// `RET`): while `008256d0` says it is not empty, calls the virtual slot
/// +0x1c4 of the first item and removes the head (`0063f7b0`).
pub fn fn_008d1560(e: &mut Engine) {
    while !e.call(LIST_IS_EMPTY, &args![PROCESS_LIST]).bool() {
        let item_address = e.call(LIST_ITEM, &args![PROCESS_LIST]).u32();
        let item = e.mem.u32(item_address);
        e.vcall(item, 0x1c4, &[]);
        e.call(LIST_REMOVE_HEAD, &args![PROCESS_LIST]);
    }
}

// Translated from 008d15b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl`, one word: when `item` is not null, hands the address of its
/// stack word to `00905330` on the list at `011dfe14`.
pub fn fn_008d15b0(e: &mut Engine, item: u32) {
    if item != 0 {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), item);
            e.call(LIST_ADD, &args![PROCESS_LIST, slot]);
        });
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_1160, fn_008d1160(Ptr)),
        entry!(0x008d_1550, fn_008d1550(Ptr, u32) -> u32),
        entry!(0x008d_1560, fn_008d1560()),
        entry!(0x008d_15b0, fn_008d15b0(u32)),
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
    fn fwd_a03_008d1160_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00c7_05c0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008d_1160, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x00c7_05c0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_008d1550() {
        assert_eq!(check(0x008d_1550, 0x008d_1130, 0x410), 0xdead_beef);
    }
}

#[cfg(test)]
mod process_list_b039_tests {
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
    fn fn_008d1560_runs_slot_0x1c4_on_each_item_until_empty() {
        let mut e = Engine::new();
        let items = e.mem.alloc(0x10);
        e.put_vtable(0x0300_0000, &[0; 0x72]);
        e.mem.set_u32(0x0300_0000 + 0x1c4, 0x0300_1000);
        e.mem.set_u32(items, 0x0300_0000);
        e.mem.set_u32(items + 8, 0x0300_0000);
        // A list of two items: the head index lives in a counter.
        let slot = e.mem.alloc(8);
        e.mem.set_u32(slot, items);
        e.mem.set_u32(slot + 4, 2);
        e.register_double(LIST_IS_EMPTY, move |e, _| Ret {
            eax: u32::from(e.mem.u32(slot + 4) == 0),
            ..Ret::default()
        });
        e.register_double(LIST_ITEM, move |_, _| Ret {
            eax: slot,
            ..Ret::default()
        });
        e.register_double(LIST_REMOVE_HEAD, move |e, _| {
            let head = e.mem.u32(slot);
            e.mem.set_u32(slot, head + 8);
            let left = e.mem.u32(slot + 4);
            e.mem.set_u32(slot + 4, left - 1);
            Ret::default()
        });
        e.register(0x0300_1000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008d_1560, &[]);
        assert_eq!(
            calls_to(&e, 0x0300_1000),
            vec![vec![items], vec![items + 8]]
        );
        assert_eq!(calls_to(&e, LIST_REMOVE_HEAD).len(), 2);
        assert!(calls_to(&e, LIST_REMOVE_HEAD)
            .iter()
            .all(|w| w == &vec![PROCESS_LIST]));
    }

    #[test]
    fn fn_008d15b0_adds_only_a_non_null_item() {
        let mut e = Engine::new();
        e.register_double(LIST_ADD, |e, a| Ret {
            eax: e.mem.u32(a[1]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x008d_15b0, &args![0u32]);
        assert!(calls_to(&e, LIST_ADD).is_empty());
        e.call(0x008d_15b0, &args![0x1234u32]);
        let calls = calls_to(&e, LIST_ADD);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], PROCESS_LIST);
    }
}
