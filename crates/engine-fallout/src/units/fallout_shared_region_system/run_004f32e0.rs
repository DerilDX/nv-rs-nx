//! `fallout shared/region system/run_004f32e0` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0066b0d0` (`thiscall`, `RET 4`): the `NiPointer` assignment.
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The two pointers `004f3310` clears (`011c7f0c`, named `DEFAULT_FULL_MASK_PROP` in
/// `tesregiondatalandscape.rs`, and `011c7ef8`).
const POINTER_011C7F0C: u32 = 0x011c_7f0c;
const POINTER_011C7EF8: u32 = 0x011c_7ef8;
/// `0096a2d0` (`thiscall`, no stack word; engine-map unit `playercharacter.cpp`): the base
/// constructor `004f3330` runs.
const BASE_CONSTRUCTOR: u32 = 0x0096_a2d0;
/// `0046ffb0` (`thiscall`): the base destructor body `004f3360` ends in.
const BASE_DESTRUCTOR: u32 = 0x0046_ffb0;
/// The list calls `004f3400` makes on `this` (`thiscall`, no stack word): the count
/// (`005ae380`, engine map `VATS::GetCount`, a body shared by folding), the head item's
/// address (`006815c0`) and the head's removal (`0063f7b0`).
const LIST_COUNT: u32 = 0x005a_e380;
const LIST_HEAD_ITEM: u32 = 0x0068_15c0;
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;

// Translated from 004f32e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`): sets a bit field of the 16-bit word
/// at `+4`: `(word & !mask) | (value << shift)`, the three arguments read as 16-bit words
/// (the shift count masked to 5 bits by `SHL`).
pub fn fn_004f32e0(e: &mut Engine, this: Ptr, value: u32, mask: u32, shift: u32) {
    let at = this.addr() + 4;
    let kept = u32::from(e.mem.u16(at)) & !(mask & 0xffff);
    let field = (value & 0xffff) << ((shift & 0xffff) & 0x1f);
    e.mem.set_u16(at, (kept | field) as u16);
}

// Translated from 004f3310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no argument): assigns null to the pointers at `011c7f0c` and
/// `011c7ef8` (`0066b0d0`), in that order.
pub fn fn_004f3310(e: &mut Engine) {
    e.call(NI_POINTER_ASSIGN, &args![POINTER_011C7F0C, 0u32]);
    e.call(NI_POINTER_ASSIGN, &args![POINTER_011C7EF8, 0u32]);
}

// Translated from 004f3330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the base constructor (`0096a2d0`), then
/// the low byte of `owns` at `+8`. Answers `this`.
pub fn fn_004f3330(e: &mut Engine, this: Ptr, owns: u32) -> Ptr {
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u8(this.addr() + 8, owns as u8);
    this
}

// Translated from 004f3400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): empties the list at `this`: while its count
/// (`005ae380`) is not 0, takes the head item (the word at the address `006815c0` answers),
/// removes the head (`0063f7b0`) and, when the byte at `+8` is set and the item is not
/// null, deletes the item (its slot 0 with 1).
pub fn fn_004f3400(e: &mut Engine, this: Ptr) {
    while e.call(LIST_COUNT, &args![this]).u32() != 0 {
        let slot = e.call(LIST_HEAD_ITEM, &args![this]).u32();
        let item = e.mem.u32(slot);
        e.call(LIST_REMOVE_HEAD, &args![this]);
        if e.mem.u8(this.addr() + 8) != 0 && item != 0 {
            e.vcall(item, 0, &args![1u32]);
        }
    }
}

// Translated from 004f3360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra matched a library destructor signature; `ECX` only):
/// the destructor: empties the list (`004f3400`), then the base destructor body
/// (`0046ffb0`).
// Translation incomplete: 004f3360 - the C++ exception frame (FS:[0] chain, unwind state 0) and the /GS cookie word are not translated.
pub fn fn_004f3360(e: &mut Engine, this: Ptr) {
    fn_004f3400(e, this);
    e.call(BASE_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_32e0, fn_004f32e0(Ptr, u32, u32, u32)),
        entry!(0x004f_3310, fn_004f3310()),
        entry!(0x004f_3330, fn_004f3330(Ptr, u32) -> Ptr),
        entry!(0x004f_3400, fn_004f3400(Ptr)),
        entry!(0x004f_3360, fn_004f3360(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn setting_a_bit_field() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u16(this + 4, 0xffff);
        e.call(0x004f_32e0, &args![this, 0x2u32, 0x0f0u32, 4u32]);
        assert_eq!(e.mem.u16(this + 4), 0xff2f);
        // Only the low words count: shift 0x10004 is 4, value 0x10001 is 1.
        e.call(
            0x004f_32e0,
            &args![this, 0x1_0001u32, 0xffff_00f0u32, 0x1_0004u32],
        );
        assert_eq!(e.mem.u16(this + 4), 0xff1f);
    }

    #[test]
    fn clearing_the_two_pointers() {
        let mut e = Engine::new();
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004f_3310, &args![]);
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN),
            vec![vec![POINTER_011C7F0C, 0], vec![POINTER_011C7EF8, 0]]
        );
    }

    #[test]
    fn construction_keeps_the_ownership_byte() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        assert_eq!(e.call(0x004f_3330, &args![this, 0x101u32]).u32(), this);
        assert_eq!(e.mem.u8(this + 8), 1);
    }

    /// A list of `count` items; the answer is the engine and the head item slot.
    fn list_engine(count: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        let left = Rc::new(Cell::new(count));
        let take = left.clone();
        e.register_double(LIST_COUNT, move |_, _| left.get().into_ret());
        e.register_double(LIST_REMOVE_HEAD, move |_, _| {
            take.set(take.get() - 1);
            Ret::default()
        });
        let slot = e.mem.alloc(4);
        e.register_double(LIST_HEAD_ITEM, move |_, _| slot.into_ret());
        e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
        e.register(0x0300_1000, |_, _| Ret::default());
        e.put_vtable(0x0300_0000, &[0x0300_1000]);
        e.call_log = Some(vec![]);
        (e, slot)
    }

    #[test]
    fn emptying_deletes_owned_items() {
        let (mut e, slot) = list_engine(2);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x0300_0000);
        e.mem.set_u32(slot, item);
        let this = e.mem.alloc(0xc);
        e.mem.set_u8(this + 8, 1);
        e.call(0x004f_3400, &args![this]);
        assert_eq!(calls(&e, LIST_REMOVE_HEAD).len(), 2);
        assert_eq!(calls(&e, 0x0300_1000), vec![vec![item, 1]; 2]);
        // Not owning: nothing deleted; a null item is skipped too.
        let (mut e, slot) = list_engine(2);
        e.mem.set_u32(slot, 0x0300_0000);
        let this = e.mem.alloc(0xc);
        e.call(0x004f_3400, &args![this]);
        let (mut e2, _) = list_engine(1);
        let owner = e2.mem.alloc(0xc);
        e2.mem.set_u8(owner + 8, 1);
        e2.call(0x004f_3400, &args![owner]);
        assert_eq!(calls(&e, LIST_REMOVE_HEAD).len(), 2);
        assert!(calls(&e, 0x0300_1000).is_empty());
        assert!(calls(&e2, 0x0300_1000).is_empty());
    }

    #[test]
    fn the_destructor_empties_then_runs_the_base() {
        let (mut e, _) = list_engine(1);
        let this = e.mem.alloc(0xc);
        e.call(0x004f_3360, &args![this]);
        assert_eq!(calls(&e, LIST_REMOVE_HEAD).len(), 1);
        assert_eq!(calls(&e, BASE_DESTRUCTOR), vec![vec![this]]);
    }
}
