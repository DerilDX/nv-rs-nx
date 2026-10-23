//! `fallout/magic/run_008229d0` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00822a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0081de70` (Ghidra name
/// `MagicProjectile::~MagicProjectile`), a destructor with an exception frame, and returns
/// nothing. Parameters: one word, `this`; the disassembly of the function ends in a plain
/// `RET` and passes only `ECX` on. `0081de70`'s disassembly ends with its exception-frame
/// unlink and returns no value (Ghidra types it void).
pub fn fn_00822a00(e: &mut Engine, this: Ptr) {
    e.call(0x0081_de70, &args![this]);
}

// Translated from 00822a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `MagicTarget` (the RTTI name of the vtable `0107bc28` it
/// installs; not named in the engine map): the vtable, the dispel list at +8
/// (constructor `0096a2d0`, which zeroes its two words), `bShowTargetStats` (+4) and
/// `bUpdating` (+5) cleared. Returns `this`.
pub fn fn_00822a40(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0107_bc28);
    e.call(0x0096_a2d0, &args![this.byte_add(8)]);
    // MagicTarget::bShowTargetStats (Xbox PDB) +0x04
    e.mem.set_u8(this.addr() + 4, 0);
    // MagicTarget::bUpdating (Xbox PDB) +0x05
    e.mem.set_u8(this.addr() + 5, 0);
    this
}

/// `MagicTarget`'s vtable (`0107bc28`, RTTI `.?AVMagicTarget@@`).
const MAGIC_TARGET_VTABLE: u32 = 0x0107_bc28;
/// The list node helpers the destructor calls (each `thiscall`, no stack words): whether a
/// node is empty (`008256d0`: its item and next both 0), the node's item slot (`006815c0`:
/// the node itself) and the next node (`00726070`: the word at +4).
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
const NODE_ITEM_SLOT: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// The free `00401030` (`cdecl`, one word).
const FREE: u32 = 0x0040_1030;
/// The list's emptying (`00470470`) and destructor body (`0046ffb0`) (`thiscall`, no stack
/// words).
const LIST_CLEAR: u32 = 0x0047_0470;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;

// Translated from 00822a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): `MagicTarget`'s destructor body
/// (the vtable it stores). Walks the dispel list at +8 from its first node until a null or
/// empty node, freeing (`00401030`) each node's item when not null; then empties the list
/// (`00470470`) and runs its destructor body (`0046ffb0`).
// Translation incomplete: 00822a80 - the C++ exception frame (FS:[0] chain, unwind state 0 for the list) is not translated.
pub fn fn_00822a80(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MAGIC_TARGET_VTABLE);
    let list = this.byte_add(8);
    let mut node = list.addr();
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(NODE_ITEM_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item != 0 {
            e.call(FREE, &args![item]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(LIST_CLEAR, &args![list]);
    e.call(LIST_DESTRUCT, &args![list]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0082_2a00, fn_00822a00(Ptr)),
        entry!(0x00822a40, fn_00822a40(Ptr) -> Ptr),
        entry!(0x0082_2a80, fn_00822a80(Ptr)),
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
    fn fwd_a03_00822a00_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0081_de70, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_2a00, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0081_de70), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_target_constructor_installs_the_vtable_and_clears_its_flags() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x0096_a2d0, |_, _| Ret::default());
        let this = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u8(this.addr() + 4, 1);
        e.mem.set_u8(this.addr() + 5, 1);
        assert_eq!(e.call(0x0082_2a40, &args![this]).ptr::<()>(), this);
        assert_eq!(e.mem.u32(this.addr()), 0x0107_bc28);
        assert_eq!(e.mem.u8(this.addr() + 4), 0);
        assert_eq!(e.mem.u8(this.addr() + 5), 0);
        let list_calls: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == 0x0096_a2d0)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(list_calls, [vec![this.addr() + 8]]);
    }
}

#[cfg(test)]
mod destructor_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NODE_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(NODE_ITEM_SLOT, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        for callee in [FREE, LIST_CLEAR, LIST_DESTRUCT] {
            e.register(callee, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn fn_00822a80_frees_the_items_then_destroys_the_list() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        // Nodes: the list head (item 0x7100), one without an item, one with 0x7300.
        let third = e.mem.alloc(8);
        e.mem.set_u32(third, 0x7300);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second + 4, third);
        e.mem.set_u32(this + 8, 0x7100);
        e.mem.set_u32(this + 12, second);
        e.call_log = Some(vec![]);
        e.call(0x0082_2a80, &args![this]);
        assert_eq!(e.mem.u32(this), MAGIC_TARGET_VTABLE);
        let log = e.call_log.take().unwrap();
        let effects: Vec<_> = log
            .iter()
            .filter(|(address, _)| [FREE, LIST_CLEAR, LIST_DESTRUCT].contains(address))
            .cloned()
            .collect();
        assert_eq!(
            effects,
            vec![
                (FREE, vec![0x7100]),
                (FREE, vec![0x7300]),
                (LIST_CLEAR, vec![this + 8]),
                (LIST_DESTRUCT, vec![this + 8]),
            ]
        );
    }

    #[test]
    fn fn_00822a80_stops_at_an_empty_head() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0082_2a80, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(address, _)| *address != FREE));
        assert!(log.iter().all(|(address, _)| *address != NODE_ITEM_SLOT));
    }
}
