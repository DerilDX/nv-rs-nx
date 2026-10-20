//! `fallout/ai/combat/hitdata.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVBGSBodyPart@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108F9E8: u32 = 0x0108_f9e8;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 009b7970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVBGSBodyPart@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_009b7970(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108F9E8);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// `00761540` (unnamed in the engine map, `creditsmenu.cpp`): reserves the next slot of a
/// `BSSimpleArray`, growing the buffer when it is full, and returns the new slot's index.
const ARRAY_RESERVE_SLOT: u32 = 0x0076_1540;
/// `00726bf0` (unnamed in the engine map): `thiscall`, `RET 8`; constructs `count` four-byte
/// elements starting at the address it is given.
const ARRAY_CONSTRUCT_ELEMENTS: u32 = 0x0072_6bf0;

// Translated from 007cb2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BGSBodyPart *, 1024>::AddUninitialized` (name from the engine map; the body
/// reserves a slot through `00761540`, constructs one element in it through `00726bf0`, then
/// copies `*item` into the slot, which is what an `Add` of one pointer does, so the name is
/// unconfirmed). `thiscall`, `RET 4`; `item` points to the pointer to store. Returns the slot's
/// index. The buffer pointer is read again after each call, as the exe does.
pub fn fn_007cb2e0(e: &mut Engine, this: Ptr<BSSimpleArray>, item: Ptr) -> u32 {
    let index = e.call(ARRAY_RESERVE_SLOT, &args![this]).u32();
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let slot = buffer.wrapping_add(index.wrapping_mul(4));
    e.call(ARRAY_CONSTRUCT_ELEMENTS, &args![this, slot, 1u32]);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let value = e.mem.u32(item.addr());
    e.mem
        .set_u32(buffer.wrapping_add(index.wrapping_mul(4)), value);
    index
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007c_b2e0, fn_007cb2e0(Ptr<BSSimpleArray>, Ptr) -> u32),
        entry!(0x009b_7970, fn_009b7970(Ptr)),
    ]
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
    fn fn_009b7970_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x009b_7970, ARRAY_VTABLE_0108F9E8, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_007cb2e0_reserves_a_slot_constructs_it_and_stores_the_pointer() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let array = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(0x20);
        e.mem.set_u32(array + 4, buffer);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0xabcd_0001);
        e.register(0x0076_1540, |_, _| 3u32.into_ret());
        e.register(0x0072_6bf0, |_, _| Ret::default());
        let index = e.call(0x007c_b2e0, &args![array, item]).u32();
        assert_eq!(index, 3);
        assert_eq!(e.mem.u32(buffer + 12), 0xabcd_0001);
        let log = e.call_log.take().unwrap();
        let reserve: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == 0x0076_1540)
            .map(|c| &c.1)
            .collect();
        assert_eq!(reserve, vec![&vec![array]]);
        let construct: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == 0x0072_6bf0)
            .map(|c| &c.1)
            .collect();
        assert_eq!(construct, vec![&vec![array, buffer + 12, 1]]);
    }

    #[test]
    fn at_007cb2e0_stores_into_the_buffer_the_reservation_moved_to() {
        let mut e = Engine::new();
        let array = e.mem.alloc(0x10);
        let old_buffer = e.mem.alloc(0x20);
        let new_buffer = e.mem.alloc(0x20);
        e.mem.set_u32(array + 4, old_buffer);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x5555_aaaa);
        e.register_double(0x0076_1540, move |e, a| {
            e.mem.set_u32(a[0] + 4, new_buffer);
            0u32.into_ret()
        });
        e.register(0x0072_6bf0, |_, _| Ret::default());
        assert_eq!(e.call(0x007c_b2e0, &args![array, item]).u32(), 0);
        assert_eq!(e.mem.u32(new_buffer), 0x5555_aaaa);
        assert_eq!(e.mem.u32(old_buffer), 0);
    }
}
