//! `fallout/misc/run_00845230` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00845b30(this, word)` (unit `bgssaveloadchangesmap`): the base
/// constructor `00845550` runs with `0x13af`.
const BASE_CONSTRUCTOR: u32 = 0x0084_5b30;
/// The vtable `00845550` stores.
const VTABLE: u32 = 0x0107_ed68;

/// `006e6da0(size, address)` (cdecl): placement allocation, returns `address`.
const PLACEMENT: u32 = 0x006e_6da0;
/// `0096a2d0(this)`: zeroes the two words of an 8-byte element (thiscall,
/// returns `this`).
const ELEMENT_INITIALIZER: u32 = 0x0096_a2d0;

// Translated from 00845300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: initializes `count` 8-byte elements starting at
/// `array` (`006e6da0(8, element)`, then `0096a2d0` unless that returned 0).
/// `thiscall` with `RET 8`: `this` (ECX) is never read.
// Translation incomplete: 00845300 - FS:[0] exception frame and security cookie (011c16bc) are not translated.
pub fn fn_00845300(e: &mut Engine, _this: Ptr, array: Ptr, count: u32) {
    for index in 0..count {
        let element = e
            .call(
                PLACEMENT,
                &args![8u32, array.addr().wrapping_add(index.wrapping_mul(8))],
            )
            .u32();
        if element != 0 {
            e.call(ELEMENT_INITIALIZER, &args![element]);
        }
    }
}

// Translated from 008453a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the pointer (+4), count (+8) and
/// capacity (+0xc), then reserves `max(capacity, count)` elements through
/// vtable slot +4 (one argument, the element count; the result is stored at
/// +4, the capacity at +0xc) and, if `count` is not zero, initializes the
/// first `count` elements with `00845300` and stores `count` at +8. `RET 8`.
pub fn fn_008453a0(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
    let t = this.addr();
    e.mem.set_u32(t + 4, 0);
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u32(t + 0xc, 0);
    let capacity = capacity.max(count);
    if capacity != 0 {
        let block = e.vcall(t, 4, &args![capacity]).u32();
        e.mem.set_u32(t + 4, block);
        e.mem.set_u32(t + 0xc, capacity);
    }
    if count != 0 {
        let array = e.mem.u32(t + 4);
        fn_00845300(e, this, Ptr::new(array), count);
        e.mem.set_u32(t + 8, count);
    }
}

// Translated from 00845550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00845b30(this, 0x13af)` and stores the
/// vtable `0107ed68` over the one it set. Returns `this`.
pub fn fn_00845550(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCTOR, &args![this, 0x13afu32]);
    e.mem.set_u32(this.addr(), VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00845300, fn_00845300(Ptr, Ptr, u32)),
        entry!(0x008453a0, fn_008453a0(Ptr, u32, u32)),
        entry!(0x00845550, fn_00845550(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn array_doubles(e: &mut Engine) {
        e.register(PLACEMENT, |_, a| a[1].into_ret());
        e.register(ELEMENT_INITIALIZER, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            a[0].into_ret()
        });
    }

    #[test]
    fn fn_00845300_initializes_each_element_and_skips_null() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        let array = e.mem.alloc(24);
        for i in 0..6 {
            e.mem.set_u32(array + 4 * i, 0xaaaa_aaaa);
        }
        e.call_log = Some(vec![]);
        e.call(0x00845300, &args![0x1111u32, array, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (PLACEMENT, vec![8, array]));
        assert_eq!(log[2], (ELEMENT_INITIALIZER, vec![array]));
        assert_eq!(log[5], (PLACEMENT, vec![8, array + 16]));
        assert_eq!(log.len(), 7);
        assert_eq!(e.mem.bytes(array, 24), vec![0; 24]);
        // A placement result of 0 skips the element initializer.
        e.register(PLACEMENT, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00845300, &args![0u32, array, 2u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
        e.call_log = Some(vec![]);
        e.call(0x00845300, &args![0u32, array, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    /// An object whose vtable slot +4 hands out blocks.
    fn array_object(e: &mut Engine) -> u32 {
        array_doubles(e);
        let vtable = e.mem.alloc(16);
        e.register(0x7000_0004, |e, a| e.mem.alloc(a[1] * 8).into_ret());
        e.mem.set_u32(vtable + 4, 0x7000_0004);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        for off in [4, 8, 0xc] {
            e.mem.set_u32(this + off, 0x99);
        }
        this
    }

    #[test]
    fn fn_008453a0_reserves_the_larger_of_capacity_and_count() {
        for (capacity, count, reserved) in [
            (0u32, 0u32, 0u32),
            (4, 0, 4),
            (2, 5, 5),
            (7, 3, 7),
            (0, 2, 2),
        ] {
            let mut e = Engine::new();
            let this = array_object(&mut e);
            e.call_log = Some(vec![]);
            e.call(0x008453a0, &args![this, capacity, count]);
            let log = e.call_log.take().unwrap();
            if reserved == 0 {
                assert_eq!(log.len(), 1, "no reservation, no initialization");
                assert_eq!(e.mem.u32(this + 4), 0);
                assert_eq!(e.mem.u32(this + 0xc), 0);
            } else {
                assert_eq!(log[1], (0x7000_0004, vec![this, reserved]));
                assert_ne!(e.mem.u32(this + 4), 0);
                assert_eq!(e.mem.u32(this + 0xc), reserved);
            }
            assert_eq!(e.mem.u32(this + 8), count);
            let initialized = log.iter().filter(|c| c.0 == ELEMENT_INITIALIZER).count();
            assert_eq!(initialized as u32, count);
        }
    }

    #[test]
    fn constructs_the_base_then_stores_its_vtable() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        let this = e.mem.alloc(16);
        e.call_log = Some(vec![]);
        let r = e.call(0x00845550, &args![this]);
        assert_eq!(r.u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (BASE_CONSTRUCTOR, vec![this, 0x13af]));
        assert_eq!(e.mem.u32(this), VTABLE, "the vtable replaces the base's");
    }
}
