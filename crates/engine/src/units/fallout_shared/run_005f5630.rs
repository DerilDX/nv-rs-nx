//! `fallout shared/run_005f5630` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005f5760 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address 0x54 bytes into the object (`ECX`, no stack words). The
/// engine map gives it no name. `TESQuest`'s `InitItem` (`0060cba0`),
/// `Save` and `Load` (`0060ceb0`) use it for the quest's condition list
/// (`TESCondition` at `quest + 0x54`); other functions call it as well.
pub fn fn_005f5760(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x54)
}

// Translated from 005f65d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the singly linked list starting at node `this` (value word at
/// `+0`, next node at `+4`; `this` may be null) contains a node whose value
/// equals the word at `wanted`. `__thiscall`, one stack word (`RET 4`).
pub fn fn_005f65d0(e: &mut Engine, this: Ptr, wanted: Ptr) -> bool {
    let mut node = this.addr();
    while node != 0 && e.mem.u32(node) != e.mem.u32(wanted.addr()) {
        node = e.mem.u32(node + 4);
    }
    node != 0
}

// Translated from 005f6610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f6610 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 7 (0x60 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005f6610(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_2c50, &args![base + 0x24]);
    e.call(0x0048_e270, &args![base + 0x2c]);
    e.call(0x0047_ff70, &args![base + 0x38]);
    e.mem.set_u32(base, 0x0104_8bb4);
    e.mem.set_u32(base + 0x18, 0x0104_8ba0);
    e.mem.set_u32(base + 0x24, 0x0104_8b88);
    e.mem.set_u32(base + 0x2c, 0x0104_8b68);
    e.mem.set_u32(base + 0x38, 0x0104_8b54);
    e.call(0x004f_15a0, &args![this, 0x7]);
    e.call(0x005f_67c0, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f5760, fn_005f5760(Ptr) -> Ptr),
        entry!(0x005f65d0, fn_005f65d0(Ptr, Ptr) -> bool),
        entry!(0x005f_6610, fn_005f6610(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    fn word(e: &mut Engine, value: u32) -> Ptr {
        let p: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(p.addr(), value);
        p
    }

    #[test]
    fn the_condition_list_is_0x54_bytes_in() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(0x0040_0000);
        assert_eq!(e.call(0x005f_5760, &args![object]).u32(), 0x0040_0054);
    }

    #[test]
    fn finds_a_value_anywhere_in_the_list() {
        let mut e = Engine::new();
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 22);
        let first: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(first.addr(), 11);
        e.mem.set_u32(first.addr() + 4, second);
        let eleven = word(&mut e, 11);
        let twenty_two = word(&mut e, 22);
        let missing = word(&mut e, 33);
        assert!(e.call(0x005f_65d0, &args![first, eleven]).bool());
        assert!(e.call(0x005f_65d0, &args![first, twenty_two]).bool());
        assert!(!e.call(0x005f_65d0, &args![first, missing]).bool());
    }

    #[test]
    fn an_empty_list_contains_nothing() {
        let mut e = Engine::new();
        let any = word(&mut e, 0);
        assert!(!e.call(0x005f_65d0, &args![Ptr::<()>::NULL, any]).bool());
    }

    #[test]
    fn fn_005f6610_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_6610,
            size: 0x60,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_2c50, &[A::T(0x24)]),
                (0x0048_e270, &[A::T(0x2c)]),
                (0x0047_ff70, &[A::T(0x38)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x7)]),
                (0x005f_67c0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_8bb4),
                (0x18, 4, 0x0104_8ba0),
                (0x24, 4, 0x0104_8b88),
                (0x2c, 4, 0x0104_8b68),
                (0x38, 4, 0x0104_8b54),
            ],
        });
    }
}
