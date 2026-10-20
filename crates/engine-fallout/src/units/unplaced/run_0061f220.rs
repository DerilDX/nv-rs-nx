//! `(unplaced)/run_0061f220` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0061f440 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0061cad0` of unit testopicinfo (clears a
/// response list), and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `0061cad0`'s translated
/// signature `fn_0061cad0(Ptr)` returns nothing.
pub fn fn_0061f440(e: &mut Engine, this: Ptr) {
    e.call(0x0061_cad0, &args![this]);
}

// Translated from 0061f260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): whether bit 4 of the
/// byte at `this + 0x25` is set.
pub fn fn_0061f260(e: &mut Engine, this: Ptr) -> bool {
    i32::from(e.mem.i8(this.addr().wrapping_add(0x25))) & 0x10 != 0
}

// Translated from 0061f220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): unless bit 4 of the
/// byte at `+0x25` is set (`0061f260`), sets the byte at `+0x22` to 1 and
/// calls the object's virtual slot `+0x48` with `0x80000000`.
pub fn fn_0061f220(e: &mut Engine, this: Ptr) {
    if !fn_0061f260(e, this) {
        e.mem.set_u8(this.addr().wrapping_add(0x22), 1);
        e.vcall(this.addr(), 0x48, &args![0x8000_0000u32]);
    }
}

// Translated from 0061f340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): stores `value` as the
/// half-word at `this + 0x20`.
pub fn fn_0061f340(e: &mut Engine, this: Ptr, value: u16) {
    e.mem.set_u16(this.addr().wrapping_add(0x20), value);
}

// Translated from 0061f420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words): whether `a` is below `b`,
/// unsigned.
pub fn fn_0061f420(_e: &mut Engine, a: u32, b: u32) -> bool {
    a < b
}

/// `008256d0` (`thiscall` on a list node, a byte), `005ae3d0` and `00905820`
/// (`thiscall`, one word: the address of the value), `006815c0`
/// (`thiscall`: the address of the node's item) and `00726070` (`thiscall`:
/// the next node), as `0061f360` uses them.
const NODE_TEST: u32 = 0x0082_56d0;
const NODE_PUT_BEFORE: u32 = 0x005a_e3d0;
const NODE_APPEND: u32 = 0x0090_5820;
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;

// Translated from 0061f360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: value, list): puts `value` into the
/// list ordered by `0061f420` (unsigned below). Nothing when either is
/// null. When `008256d0` marks the head, `005ae3d0` on it takes the value.
/// Otherwise each node in turn: stop when it is null or `008256d0` marks
/// it; stop when its item (`006815c0`) is already the value; when the item
/// is below the value `005ae3d0` on that node takes it; at the last node
/// (`00726070` answers null) `00905820` takes it. The two callees get the
/// address of the `value` argument.
pub fn fn_0061f360(e: &mut Engine, value: u32, list: Ptr) {
    if value == 0 || list.is_null() {
        return;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        if e.call(NODE_TEST, &args![list]).bool() {
            e.call(NODE_PUT_BEFORE, &args![list, slot]);
            return;
        }
        let mut node = list.addr();
        while node != 0 {
            if e.call(NODE_TEST, &args![node]).bool() {
                return;
            }
            let item_slot = e.call(NODE_ITEM, &args![node]).u32();
            let item = e.mem.u32(item_slot);
            let value = e.mem.u32(slot.addr());
            if item == value {
                return;
            }
            if fn_0061f420(e, item, value) {
                e.call(NODE_PUT_BEFORE, &args![node, slot]);
                return;
            }
            if e.call(NODE_NEXT, &args![node]).u32() == 0 {
                e.call(NODE_APPEND, &args![node, slot]);
                return;
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
}

/// The vtables `0102f13c` (RTTI `.?AVhkpEntityListener@@`), `0104d91c`
/// (RTTI `.?AVbhkEntityListener@@`) and `0104d8e8` (RTTI
/// `.?AVBGSAcousticSpaceListener@@`).
const HKP_ENTITY_LISTENER_VTABLE: u32 = 0x0102_f13c;
const BHK_ENTITY_LISTENER_VTABLE: u32 = 0x0104_d91c;
const ACOUSTIC_SPACE_LISTENER_VTABLE: u32 = 0x0104_d8e8;

// Translated from 0061f580 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `hkpEntityListener` constructor (unnamed in the engine map;
/// `thiscall`, plain `RET`): stores the vtable `0102f13c`. Returns `this`.
pub fn fn_0061f580(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), HKP_ENTITY_LISTENER_VTABLE);
    this
}

// Translated from 0061f4d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061f4d0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// A `bhkEntityListener` constructor (unnamed in the engine map;
/// `thiscall`, `RET 4`): the `hkpEntityListener` constructor (`0061f580`),
/// the vtable `0104d91c`, `004ee810` on `this + 0xc`, `owner` at `+4` and
/// the byte at `+8` set to 1. Returns `this`.
pub fn fn_0061f4d0(e: &mut Engine, this: Ptr, owner: u32) -> Ptr {
    fn_0061f580(e, this);
    e.mem.set_u32(this.addr(), BHK_ENTITY_LISTENER_VTABLE);
    e.call(0x004e_e810, &args![this.addr().wrapping_add(0xc)]);
    e.mem.set_u32(this.addr().wrapping_add(4), owner);
    e.mem.set_u8(this.addr().wrapping_add(8), 1);
    this
}

// Translated from 0061f460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061f460 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// A `BGSAcousticSpaceListener` constructor (unnamed in the engine map;
/// `thiscall`, plain `RET`): the `bhkEntityListener` constructor
/// (`0061f4d0` with 0), the vtable `0104d8e8`, `0061fa10` with `0x25` on
/// `this + 0x18`, and the byte at `+0x14` set to 1. Returns `this`.
pub fn fn_0061f460(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0061f4d0(e, this, 0);
    e.mem.set_u32(this.addr(), ACOUSTIC_SPACE_LISTENER_VTABLE);
    e.call(0x0061_fa10, &args![this.addr().wrapping_add(0x18), 0x25u32]);
    e.mem.set_u8(this.addr().wrapping_add(0x14), 1);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_f440, fn_0061f440(Ptr)),
        entry!(0x0061_f260, fn_0061f260(Ptr) -> bool),
        entry!(0x0061_f220, fn_0061f220(Ptr)),
        entry!(0x0061_f340, fn_0061f340(Ptr, u16)),
        entry!(0x0061_f420, fn_0061f420(u32, u32) -> bool),
        entry!(0x0061_f360, fn_0061f360(u32, Ptr)),
        entry!(0x0061_f580, fn_0061f580(Ptr) -> Ptr),
        entry!(0x0061_f4d0, fn_0061f4d0(Ptr, u32) -> Ptr),
        entry!(0x0061_f460, fn_0061f460(Ptr) -> Ptr),
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
    fn fwd_a03_0061f440_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0061_cad0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0061_f440, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0061_cad0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod b015_tests {
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
    fn bit_4_of_byte_25_decides_whether_the_slot_runs() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        e.put_vtable(0x0090_0000, &[0x0090_1000; 0x13]);
        e.mem.set_u32(this, 0x0090_0000);
        e.register(0x0090_1000, |_, _| Ret::default());
        e.mem.set_u8(this + 0x25, 0xef);
        assert!(!e.call(0x0061_f260, &[this]).bool());
        e.call_log = Some(vec![]);
        e.call(0x0061_f220, &[this]);
        assert_eq!(calls_to(&e, 0x0090_1000), vec![vec![this, 0x8000_0000]]);
        assert_eq!(e.mem.u8(this + 0x22), 1);
        e.mem.set_u8(this + 0x25, 0x10);
        e.mem.set_u8(this + 0x22, 0);
        assert!(e.call(0x0061_f260, &[this]).bool());
        e.call_log = Some(vec![]);
        e.call(0x0061_f220, &[this]);
        assert!(calls_to(&e, 0x0090_1000).is_empty());
        assert_eq!(e.mem.u8(this + 0x22), 0);
    }

    #[test]
    fn half_word_setter_and_unsigned_compare() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        e.call(0x0061_f340, &[this, 0x1_beef]);
        assert_eq!(e.mem.u16(this + 0x20), 0xbeef);
        assert!(e.call(0x0061_f420, &[1, 2]).bool());
        assert!(!e.call(0x0061_f420, &[2, 2]).bool());
        assert!(!e.call(0x0061_f420, &[0xffff_ffff, 2]).bool());
    }

    /// A list of nodes (item at +0, next at +4, "marked" at +8) and doubles
    /// for the node callees; returns the head.
    fn list(e: &mut Engine, items: &[u32], marked_head: bool) -> u32 {
        let mut next = 0u32;
        for item in items.iter().rev() {
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        e.mem.set_u32(next + 8, u32::from(marked_head));
        e.register(NODE_TEST, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_PUT_BEFORE, |_, _| Ret::default());
        e.register(NODE_APPEND, |_, _| Ret::default());
        next
    }

    /// The (callee, node, value) of each insertion call.
    fn insertions(e: &Engine) -> Vec<(u32, u32, u32)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == NODE_PUT_BEFORE || *a == NODE_APPEND)
            .map(|(a, w)| (*a, w[0], w[1]))
            .collect()
    }

    #[test]
    fn ordered_insert_puts_the_value_before_the_first_smaller_item() {
        let mut e = Engine::new();
        let head = list(&mut e, &[9, 7, 3], false);
        let third = e.mem.u32(e.mem.u32(head + 4) + 4);
        e.call_log = Some(vec![]);
        e.call(0x0061_f360, &[5, head]);
        let done = insertions(&e);
        assert_eq!(done.len(), 1);
        assert_eq!((done[0].0, done[0].1), (NODE_PUT_BEFORE, third));
        // The callee gets the address of a word holding the value.
        assert_ne!(done[0].2, 5);
        // Already there: nothing; smaller than all: appended at the end.
        e.call_log = Some(vec![]);
        e.call(0x0061_f360, &[7, head]);
        assert!(insertions(&e).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0061_f360, &[1, head]);
        let done = insertions(&e);
        assert_eq!((done[0].0, done[0].1), (NODE_APPEND, third));
    }

    #[test]
    fn ordered_insert_with_a_marked_head_or_null_arguments() {
        let mut e = Engine::new();
        let head = list(&mut e, &[9], true);
        e.call_log = Some(vec![]);
        e.call(0x0061_f360, &[5, head]);
        let done = insertions(&e);
        assert_eq!((done[0].0, done[0].1), (NODE_PUT_BEFORE, head));
        e.call_log = Some(vec![]);
        e.call(0x0061_f360, &[0, head]);
        e.call(0x0061_f360, &[5, 0]);
        assert!(insertions(&e).is_empty());
        assert!(calls_to(&e, NODE_TEST).is_empty());
    }

    #[test]
    fn listener_constructors_chain_their_bases() {
        let mut e = Engine::new();
        e.register(0x004e_e810, |_, _| Ret::default());
        e.register(0x0061_fa10, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x0061_f580, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), HKP_ENTITY_LISTENER_VTABLE);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_f4d0, &[this, 0x77]).u32(), this);
        assert_eq!(e.mem.u32(this), BHK_ENTITY_LISTENER_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x77);
        assert_eq!(e.mem.u8(this + 8), 1);
        assert_eq!(calls_to(&e, 0x004e_e810), vec![vec![this + 0xc]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_f460, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), ACOUSTIC_SPACE_LISTENER_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u8(this + 0x14), 1);
        assert_eq!(calls_to(&e, 0x0061_fa10), vec![vec![this + 0x18, 0x25]]);
    }
}
