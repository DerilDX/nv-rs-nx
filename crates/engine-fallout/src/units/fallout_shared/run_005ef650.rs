//! `fallout shared/run_005ef650` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSSimpleArray` template members: the `unsigned short` array's element
//! construction and resize (`005ef650`, `005ef6a0`) and the foot IK leg
//! data array's start (`005ef910`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// Places one element of `size` bytes at an address (`006e6da0`, cdecl:
/// size, address).
const PLACE_ELEMENT: u32 = 0x006e_6da0;
/// The leg data array's element construction (`005ef750`, `RET 8`:
/// address, count).
const CONSTRUCT_LEG_ELEMENTS: u32 = 0x005e_f750;

// Translated from 005ef650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs `count` `unsigned short` elements from `elements`
/// (`006e6da0(2, address)` for each; `this` is not read).
pub fn fn_005ef650(e: &mut Engine, _this: Ptr, elements: u32, count: u32) {
    let mut i = 0u32;
    while i < count {
        e.call(
            PLACE_ELEMENT,
            &args![2u32, elements.wrapping_add(i.wrapping_mul(2))],
        );
        i = i.wrapping_add(1);
    }
}

// Translated from 005ef6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resizes the `unsigned short` array's buffer to `capacity` keeping
/// `count` elements (`RET 8`): without a buffer, allocates one (vtable +4)
/// and sets the capacity; with `count` equal to the capacity, reallocates
/// in place (vtable +0xC with the buffer and `capacity`; the capacity
/// field is left); otherwise allocates a new buffer, moves the elements
/// (`005efdd0`), frees the old one (`006a8500`) and keeps the new one.
pub fn fn_005ef6a0(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, count: u32) {
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    if buffer == 0 {
        let new = e.vcall(this.addr(), 4, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, new);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    } else if count == e.get(this, BSSimpleArray::iReservedSize) {
        let new = e.vcall(this.addr(), 0xc, &args![buffer, capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, new);
    } else {
        let new = e.vcall(this.addr(), 4, &args![capacity]).u32();
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(0x005e_fdd0, &args![this, new, old, count]);
        e.call(0x006a_8500, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, new);
    }
}

// Translated from 005ef910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts the foot IK leg data array (`RET 8`): empty, then with a
/// capacity of `capacity` (at least `count`; allocated through vtable +4
/// when not 0) and `count` constructed elements.
pub fn fn_005ef910(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, count: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let capacity = if count > capacity { count } else { capacity };
    if capacity > 0 {
        let buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    }
    if count > 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(CONSTRUCT_LEG_ELEMENTS, &args![this, buffer, count]);
        e.set(this, BSSimpleArray::iSize, count);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_f650, fn_005ef650(Ptr, u32, u32)),
        entry!(0x005e_f6a0, fn_005ef6a0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x005e_f910, fn_005ef910(Ptr<BSSimpleArray>, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An array whose vtable +4 allocates 0x9000 and +0xC reallocates to
    /// 0xa000.
    fn array(e: &mut Engine, buffer: u32, count: u32, capacity: u32) -> Ptr<BSSimpleArray> {
        let this: Ptr<BSSimpleArray> = e.new_object();
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x7000_0004);
        e.mem.set_u32(vtable + 0xc, 0x7000_000c);
        e.mem.set_u32(this.addr(), vtable);
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iSize, count);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
        e.register(0x7000_0004, |_, _| ret(0x9000));
        e.register(0x7000_000c, |_, _| ret(0xa000));
        for addr in [
            0x005e_fdd0u32,
            0x006a_8500,
            CONSTRUCT_LEG_ELEMENTS,
            PLACE_ELEMENT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        this
    }

    fn state(e: &Engine, this: Ptr<BSSimpleArray>) -> (u32, u32, u32) {
        (
            e.get(this, BSSimpleArray::pBuffer),
            e.get(this, BSSimpleArray::iSize),
            e.get(this, BSSimpleArray::iReservedSize),
        )
    }

    #[test]
    fn fn_005ef650_places_each_element() {
        let mut e = Engine::new();
        let this = array(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_f650, &args![this, 0x5000u32, 3u32]);
        assert_eq!(
            calls(&e, PLACE_ELEMENT),
            vec![vec![2, 0x5000], vec![2, 0x5002], vec![2, 0x5004]]
        );
        e.call_log = Some(vec![]);
        e.call(0x005e_f650, &args![this, 0x5000u32, 0u32]);
        assert!(calls(&e, PLACE_ELEMENT).is_empty());
    }

    #[test]
    fn fn_005ef6a0_allocates_reallocates_or_moves() {
        let mut e = Engine::new();
        let this = array(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_f6a0, &args![this, 8u32, 0u32]);
        assert_eq!(state(&e, this), (0x9000, 0, 8));
        assert_eq!(calls(&e, 0x7000_0004), vec![vec![this.addr(), 8]]);

        let mut e = Engine::new();
        let this = array(&mut e, 0x5000, 4, 4);
        e.call_log = Some(vec![]);
        e.call(0x005e_f6a0, &args![this, 8u32, 4u32]);
        assert_eq!(calls(&e, 0x7000_000c), vec![vec![this.addr(), 0x5000, 8]]);
        assert_eq!(state(&e, this), (0xa000, 4, 4));

        let mut e = Engine::new();
        let this = array(&mut e, 0x5000, 3, 4);
        e.call_log = Some(vec![]);
        e.call(0x005e_f6a0, &args![this, 8u32, 3u32]);
        assert_eq!(
            calls(&e, 0x005e_fdd0),
            vec![vec![this.addr(), 0x9000, 0x5000, 3]]
        );
        assert_eq!(calls(&e, 0x006a_8500), vec![vec![this.addr()]]);
        assert_eq!(state(&e, this), (0x9000, 3, 4));
    }

    #[test]
    fn fn_005ef910_starts_with_capacity_and_count() {
        let mut e = Engine::new();
        let this = array(&mut e, 0x5000, 3, 4);
        e.call_log = Some(vec![]);
        e.call(0x005e_f910, &args![this, 0u32, 0u32]);
        assert_eq!(state(&e, this), (0, 0, 0));
        assert!(calls(&e, 0x7000_0004).is_empty());

        // The count raises the capacity.
        e.call_log = Some(vec![]);
        e.call(0x005e_f910, &args![this, 2u32, 5u32]);
        assert_eq!(calls(&e, 0x7000_0004), vec![vec![this.addr(), 5]]);
        assert_eq!(
            calls(&e, CONSTRUCT_LEG_ELEMENTS),
            vec![vec![this.addr(), 0x9000, 5]]
        );
        assert_eq!(state(&e, this), (0x9000, 5, 5));

        e.call_log = Some(vec![]);
        e.call(0x005e_f910, &args![this, 6u32, 0u32]);
        assert_eq!(state(&e, this), (0x9000, 0, 6));
        assert!(calls(&e, CONSTRUCT_LEG_ELEMENTS).is_empty());
    }
}
