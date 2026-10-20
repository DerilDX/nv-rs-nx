//! `fallout/interface/menus/creditsmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// The handle of the empty `NiFixedString`.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;

// Translated from 007613f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the map files it under `creditsmenu.cpp`, a
/// range placement; the body is not menu code): stores the handle of the empty
/// `NiFixedString` (the word at `0109b220`) in the word at `this` and returns
/// `this`, the default constructor of an `NiFixedString`-sized object (the stream
/// code uses it as the element constructor of its fixed string array). The
/// decompiler shows the stored value as 0; the disassembly reads the global.
pub fn fn_007613f0(e: &mut Engine, this: Ptr) -> Ptr {
    let empty = e.global::<u32>(EMPTY_FIXED_STRING);
    e.mem.set_u32(this.addr(), empty);
    this
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNiFixedString@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_01072514: u32 = 0x0107_2514;
/// `007615d0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_007615D0: u32 = 0x0076_15d0;

// Translated from 007614a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNiFixedString@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `007615d0(this, 1)`.
pub fn fn_007614a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01072514);
    e.call(ARRAY_RELEASE_007615D0, &args![this, 1u32]);
}

/// `00438b90` (unnamed in the engine map): `thiscall`; whether `iSize == iReservedSize` (the
/// array is full), in `AL`.
const ARRAY_IS_FULL: u32 = 0x0043_8b90;
/// `009a3910` (unnamed in the engine map): `thiscall`; the next capacity: `iReservedSize + 0x400`
/// when it is above `0x400`, otherwise twice `iReservedSize`.
const ARRAY_NEXT_CAPACITY: u32 = 0x009a_3910;
/// `0042fc00` (unnamed in the engine map): `thiscall`, `RET 8`; reallocates the buffer to the
/// given capacity, keeping the given number of elements.
const ARRAY_REALLOCATE: u32 = 0x0042_fc00;
/// Vtable slot (byte offset) that allocates a block of the requested number of elements.
const SLOT_ALLOCATE: u32 = 0x04;

// Translated from 00761540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`creditsmenu.cpp`; the body is a generic `BSSimpleArray` slot
/// reservation, folded from the template). `__fastcall`: when the array is full, allocates a
/// four-element buffer through virtual slot +4 if it has no capacity, otherwise reallocates to
/// the next capacity; then counts one more element and answers the index of the new slot.
pub fn fn_00761540(e: &mut Engine, this: Ptr<BSSimpleArray>) -> u32 {
    if e.call(ARRAY_IS_FULL, &args![this]).bool() {
        if e.get(this, BSSimpleArray::iReservedSize) == 0 {
            let capacity = 4u32;
            let buffer = e.vcall(this.addr(), SLOT_ALLOCATE, &args![capacity]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, capacity);
        } else {
            let capacity = e.call(ARRAY_NEXT_CAPACITY, &args![this]).u32();
            let size = e.get(this, BSSimpleArray::iSize);
            e.call(ARRAY_REALLOCATE, &args![this, capacity, size]);
            e.set(this, BSSimpleArray::iReservedSize, capacity);
        }
    }
    let size = e.get(this, BSSimpleArray::iSize).wrapping_add(1);
    e.set(this, BSSimpleArray::iSize, size);
    size.wrapping_sub(1)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0076_1540, fn_00761540(Ptr<BSSimpleArray>) -> u32),
        entry!(0x0076_13f0, fn_007613f0(Ptr) -> Ptr),
        entry!(0x0076_14a0, fn_007614a0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_word_becomes_the_empty_string_handle() {
        let mut e = Engine::new();
        e.map(EMPTY_FIXED_STRING & !0xfff, 0x1000);
        e.set_global(EMPTY_FIXED_STRING, 0x0123_4568u32);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0xdead_beef);
        assert_eq!(e.call(0x0076_13f0, &args![object]).u32(), object);
        assert_eq!(e.mem.u32(object), 0x0123_4568);
    }
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_007614a0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0076_14a0, ARRAY_VTABLE_01072514, ARRAY_RELEASE_007615D0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
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

    fn rig(vtable: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.put_vtable(vtable, &[0, 0x0f00_7000]);
        e.register(0x0f00_7000, |_, a| (0x3000_0000 + a[1]).into_ret());
        e.register(ARRAY_IS_FULL, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) == e.mem.u32(a[0] + 0xc)).into_ret()
        });
        e.register(ARRAY_NEXT_CAPACITY, |_, _| 0x40u32.into_ret());
        e.register(ARRAY_REALLOCATE, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, vtable);
        (e, array)
    }

    #[test]
    fn at_00761540_allocates_four_elements_for_an_array_without_capacity() {
        let (mut e, array) = rig(0x0f00_6100);
        assert_eq!(e.call(0x0076_1540, &args![array]).u32(), 0);
        assert_eq!(calls(&e, 0x0f00_7000), vec![vec![array, 4]]);
        assert_eq!(e.mem.u32(array + 4), 0x3000_0004);
        assert_eq!(e.mem.u32(array + 8), 1);
        assert_eq!(e.mem.u32(array + 0xc), 4);
        assert!(calls(&e, ARRAY_NEXT_CAPACITY).is_empty());
    }

    #[test]
    fn at_00761540_reallocates_a_full_array_to_the_next_capacity() {
        let (mut e, array) = rig(0x0f00_6200);
        e.mem.set_u32(array + 4, 0x4000);
        e.mem.set_u32(array + 8, 4);
        e.mem.set_u32(array + 0xc, 4);
        assert_eq!(e.call(0x0076_1540, &args![array]).u32(), 4);
        assert_eq!(calls(&e, ARRAY_NEXT_CAPACITY), vec![vec![array]]);
        assert_eq!(calls(&e, ARRAY_REALLOCATE), vec![vec![array, 0x40, 4]]);
        assert_eq!(e.mem.u32(array + 8), 5);
        assert_eq!(e.mem.u32(array + 0xc), 0x40);
        // The buffer pointer is the reallocation's business, not this function's.
        assert_eq!(e.mem.u32(array + 4), 0x4000);
    }

    #[test]
    fn at_00761540_only_counts_when_there_is_room() {
        let (mut e, array) = rig(0x0f00_6300);
        e.mem.set_u32(array + 4, 0x4000);
        e.mem.set_u32(array + 8, 2);
        e.mem.set_u32(array + 0xc, 8);
        assert_eq!(e.call(0x0076_1540, &args![array]).u32(), 2);
        assert_eq!(e.mem.u32(array + 8), 3);
        assert_eq!(e.mem.u32(array + 0xc), 8);
        assert!(calls(&e, ARRAY_NEXT_CAPACITY).is_empty());
        assert!(calls(&e, ARRAY_REALLOCATE).is_empty());
        assert!(calls(&e, 0x0f00_7000).is_empty());
    }
}
