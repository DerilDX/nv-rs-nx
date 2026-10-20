//! `fallout shared/run_004b0aa0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `008256d0` (`thiscall`, no stack word): the stop test of the walk (low byte of `EAX`).
const WALK_STOPS: u32 = 0x0082_56d0;
/// `006815c0` (`thiscall`, no stack word): the address of the word holding the entry to test.
const ENTRY_HOLDER: u32 = 0x0068_15c0;
/// `00726070` (`thiscall`, no stack word; unit `fallout/interface/menumanager.cpp`): the next
/// object of the walk.
const WALK_NEXT: u32 = 0x0072_6070;
/// `004a4460` (`thiscall`, no stack word): the emptiness test of the list at +4 of the entry
/// (low byte of `EAX`).
const LIST_IS_EMPTY: u32 = 0x004a_4460;
/// `004ee8a0` (`thiscall`, no stack word; engine map
/// `BSSimpleList<NiPointer<BSTempEffect>>::RemoveHead`).
const LIST_REMOVE_HEAD: u32 = 0x004e_e8a0;
/// `00905330` (`thiscall`, `RET 4`; unit `fallout/ai/highprocess.cpp`): given the address of
/// the found entry.
const HAND_OVER_ENTRY: u32 = 0x0090_5330;
/// `004b08e0` (`thiscall`, `RET 4`).
const ENTRY_FINISH: u32 = 0x004b_08e0;

// Translated from 004b0aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): for a nonzero `key`, walks from `this`
/// (`00726070` gives the next object) while the object is not null, nothing is found yet and
/// `008256d0` says go on; an object whose entry (the word at the address `006815c0` gives)
/// is set and starts with `key` is the found entry. With an entry found, removes up to
/// `count` heads of its list at +4 (`004ee8a0`) while that list is not empty (`004a4460`);
/// when the list is then empty, passes the address of the found entry to `00905330` on
/// `this`, and if the entry (as `00905330` left it) is not null calls `004b08e0(1)` on it.
pub fn fn_004b0aa0(e: &mut Engine, this: Ptr, key: u32, count: u32) {
    if key == 0 {
        return;
    }
    e.with_stack(4, |e, found| {
        e.mem.set_u32(found.addr(), 0);
        let mut current = this.addr();
        while current != 0
            && e.mem.u32(found.addr()) == 0
            && !e.call(WALK_STOPS, &args![current]).bool()
        {
            let holder = e.call(ENTRY_HOLDER, &args![current]).u32();
            let entry = e.mem.u32(holder);
            if entry != 0 && e.mem.u32(entry) == key {
                e.mem.set_u32(found.addr(), entry);
            }
            current = e.call(WALK_NEXT, &args![current]).u32();
        }
        let entry = e.mem.u32(found.addr());
        if entry == 0 {
            return;
        }
        let list = entry.wrapping_add(4);
        let mut removed = 0u32;
        while removed < count && !e.call(LIST_IS_EMPTY, &args![list]).bool() {
            e.call(LIST_REMOVE_HEAD, &args![list]);
            removed = removed.wrapping_add(1);
        }
        if e.call(LIST_IS_EMPTY, &args![list]).bool() {
            e.call(HAND_OVER_ENTRY, &args![this, found]);
            let entry = e.mem.u32(found.addr());
            if entry != 0 {
                e.call(ENTRY_FINISH, &args![entry, 1u32]);
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004b_0aa0, fn_004b0aa0(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    /// Three objects 0x100, 0x200, 0x300 in a chain; object `n`'s entry holder is
    /// `holders[n]`; the list at +4 of the entry empties after `heads` removals.
    fn fixture(heads: u32) -> (Engine, [u32; 3], Rc<Cell<u32>>) {
        let mut e = Engine::new();
        let holders = [e.mem.alloc(4), e.mem.alloc(4), e.mem.alloc(4)];
        let chain = holders;
        e.register(WALK_STOPS, |_, _| 0u32.into_ret());
        e.register_double(ENTRY_HOLDER, move |_, a| {
            chain[(a[0] / 0x100 - 1) as usize].into_ret()
        });
        e.register(WALK_NEXT, |_, a| {
            (if a[0] == 0x300 { 0 } else { a[0] + 0x100 }).into_ret()
        });
        let left = Rc::new(Cell::new(heads));
        let in_test = left.clone();
        let counter = left.clone();
        e.register_double(LIST_IS_EMPTY, move |_, _| (counter.get() == 0).into_ret());
        e.register_double(LIST_REMOVE_HEAD, move |_, _| {
            in_test.set(in_test.get() - 1);
            Ret::default()
        });
        e.register(HAND_OVER_ENTRY, |_, _| Ret::default());
        e.register(ENTRY_FINISH, |_, _| Ret::default());
        (e, holders, left)
    }

    #[test]
    fn finds_the_entry_empties_its_list_and_hands_it_over() {
        let (mut e, holders, _) = fixture(2);
        let entry = e.mem.alloc(0xc);
        e.mem.set_u32(entry, 0x77);
        e.mem.set_u32(holders[1], entry);
        e.call_log = Some(vec![]);
        e.call(0x004b_0aa0, &args![0x100u32, 0x77u32, 5u32]);
        let log = e.call_log.take().unwrap();
        // The walk stops after the second object (found).
        assert_eq!(log.iter().filter(|c| c.0 == WALK_NEXT).count(), 2);
        assert_eq!(log.iter().filter(|c| c.0 == LIST_REMOVE_HEAD).count(), 2);
        let handed: Vec<_> = log.iter().filter(|c| c.0 == HAND_OVER_ENTRY).collect();
        assert_eq!(handed.len(), 1);
        assert_eq!(handed[0].1[0], 0x100);
        assert!(log.contains(&(ENTRY_FINISH, vec![entry, 1])));
    }

    #[test]
    fn a_list_left_with_heads_is_not_handed_over() {
        let (mut e, holders, left) = fixture(5);
        let entry = e.mem.alloc(0xc);
        e.mem.set_u32(entry, 0x77);
        e.mem.set_u32(holders[2], entry);
        e.call_log = Some(vec![]);
        e.call(0x004b_0aa0, &args![0x100u32, 0x77u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == LIST_REMOVE_HEAD).count(), 3);
        assert_eq!(left.get(), 2);
        assert!(log.iter().all(|c| c.0 != HAND_OVER_ENTRY));
    }

    #[test]
    fn no_key_or_no_match_does_nothing_more() {
        let (mut e, _, _) = fixture(0);
        e.call_log = Some(vec![]);
        e.call(0x004b_0aa0, &args![0x100u32, 0u32, 3u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x004b_0aa0, &args![0x100u32, 0x77u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == WALK_NEXT).count(), 3);
        assert!(log.iter().all(|c| c.0 != LIST_IS_EMPTY));
    }
}
