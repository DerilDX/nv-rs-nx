//! `fallout/misc/run_00869600` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00869a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00869a40`, which stores the vtable `010827f0` at
/// `+0` and frees the block held at `+4` through `008696c0`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00869a40`'s disassembly ends with the stack cleanup of that cdecl
/// call (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_00869a20(e: &mut Engine, this: Ptr) {
    e.call(0x0086_9a40, &args![this]);
}

// Translated from 00869b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00869b90`, which stores the vtable `01082800` at
/// `+0` and frees the block held at `+4` through `008696c0`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00869b90`'s disassembly ends with the stack cleanup of that cdecl
/// call (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_00869b70(e: &mut Engine, this: Ptr) {
    e.call(0x0086_9b90, &args![this]);
}

// Translated from 00869cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00869ce0`, which stores the vtable `01082810` at
/// `+0` and frees the block held at `+4` through `008696c0`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00869ce0`'s disassembly ends with the stack cleanup of that cdecl
/// call (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_00869cc0(e: &mut Engine, this: Ptr) {
    e.call(0x0086_9ce0, &args![this]);
}

// Translated from 00869e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00869e30`, which stores the vtable `01082820` at
/// `+0` and destroys the array held at `+4` through `004b0220` (the vector deleting
/// destructor), and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `00869e30`'s disassembly ends
/// with the stack cleanup of that call (`ADD ESP,4`); `004b0220` stores its own result only
/// in a frame slot, so no value is returned (Ghidra types both void).
pub fn fn_00869e10(e: &mut Engine, this: Ptr) {
    e.call(0x0086_9e30, &args![this]);
}

/// The five-constructor group: each runs a base body on `this` with its two stack words, then
/// stores a vtable (all four RTTI classes are `NiTObjectArray<...>` instances), except
/// `00869f00`, which stores its vtable first and then calls the shared body `006b3eb0`.
const BASE_BODY_00869AA0: u32 = 0x0086_9aa0;
const BASE_BODY_00869BF0: u32 = 0x0086_9bf0;
const BASE_BODY_00869D40: u32 = 0x0086_9d40;
const BASE_BODY_00869E90: u32 = 0x0086_9e90;
const BASE_BODY_006B3EB0: u32 = 0x006b_3eb0;
const VTABLE_008699C0: u32 = 0x0108_27e8;
const VTABLE_00869B10: u32 = 0x0108_27f8;
const VTABLE_00869C60: u32 = 0x0108_2808;
const VTABLE_00869DB0: u32 = 0x0108_2818;
const VTABLE_00869F00: u32 = 0x0106_ba6c;

// Translated from 008699c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00869aa0(this, first, second)`, then stores the vtable
/// `010827e8` (RTTI class `NiTObjectArray<TESObjectREFR*>`). Returns `this`.
pub fn fn_008699c0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(BASE_BODY_00869AA0, &args![this, first, second]);
    e.mem.set_u32(this.addr(), VTABLE_008699C0);
    this
}

// Translated from 00869b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00869bf0(this, first, second)`, then stores the vtable
/// `010827f8` (RTTI class `NiTObjectArray<BipedAnim*>`). Returns `this`.
pub fn fn_00869b10(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(BASE_BODY_00869BF0, &args![this, first, second]);
    e.mem.set_u32(this.addr(), VTABLE_00869B10);
    this
}

// Translated from 00869c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00869d40(this, first, second)`, then stores the vtable
/// `01082808` (RTTI class `NiTObjectArray<Animation*>`). Returns `this`.
pub fn fn_00869c60(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(BASE_BODY_00869D40, &args![this, first, second]);
    e.mem.set_u32(this.addr(), VTABLE_00869C60);
    this
}

// Translated from 00869db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00869e90(this, first, second)`, then stores the vtable
/// `01082818` (RTTI class `NiTObjectArray<NiPointer<BSTempEffect>>`). Returns `this`.
pub fn fn_00869db0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(BASE_BODY_00869E90, &args![this, first, second]);
    e.mem.set_u32(this.addr(), VTABLE_00869DB0);
    this
}

// Translated from 00869f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `0106ba6c` (RTTI class
/// `BSSimpleArray<NavMesh*,...>`), then runs `006b3eb0(this, first, second)`. Returns `this`.
pub fn fn_00869f00(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_00869F00);
    e.call(BASE_BODY_006B3EB0, &args![this, first, second]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0086_9a20, fn_00869a20(Ptr)),
        entry!(0x0086_9b70, fn_00869b70(Ptr)),
        entry!(0x0086_9cc0, fn_00869cc0(Ptr)),
        entry!(0x0086_9e10, fn_00869e10(Ptr)),
        entry!(0x0086_99c0, fn_008699c0(Ptr, u32, u32) -> Ptr),
        entry!(0x0086_9b10, fn_00869b10(Ptr, u32, u32) -> Ptr),
        entry!(0x0086_9c60, fn_00869c60(Ptr, u32, u32) -> Ptr),
        entry!(0x0086_9db0, fn_00869db0(Ptr, u32, u32) -> Ptr),
        entry!(0x0086_9f00, fn_00869f00(Ptr, u32, u32) -> Ptr),
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
    fn fwd_a03_00869a20_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0086_9a40, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0086_9a20, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0086_9a40), vec![vec![0x5000]]);
    }

    #[test]
    fn fwd_a03_00869b70_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0086_9b90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0086_9b70, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0086_9b90), vec![vec![0x5000]]);
    }

    #[test]
    fn fwd_a03_00869cc0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0086_9ce0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0086_9cc0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0086_9ce0), vec![vec![0x5000]]);
    }

    #[test]
    fn fwd_a03_00869e10_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0086_9e30, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0086_9e10, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0086_9e30), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn four_constructors_run_the_body_then_store_their_vtable() {
        for (entry, body, vtable) in [
            (0x0086_99c0, BASE_BODY_00869AA0, VTABLE_008699C0),
            (0x0086_9b10, BASE_BODY_00869BF0, VTABLE_00869B10),
            (0x0086_9c60, BASE_BODY_00869D40, VTABLE_00869C60),
            (0x0086_9db0, BASE_BODY_00869E90, VTABLE_00869DB0),
        ] {
            let mut e = Engine::new();
            e.register(body, |e, a| {
                e.mem.set_u32(a[0], 0x1234);
                Ret::default()
            });
            let this = e.mem.alloc(0x20);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(entry, &args![this, 3u32, 4u32]).u32(), this);
            assert_eq!(calls_to(&mut e, body), vec![vec![this, 3, 4]]);
            assert_eq!(e.mem.u32(this), vtable);
        }
    }

    #[test]
    fn the_fifth_stores_the_vtable_before_calling_the_body() {
        let mut e = Engine::new();
        e.register(BASE_BODY_006B3EB0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 8, seen);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0086_9f00, &args![this, 5u32, 6u32]).u32(), this);
        assert_eq!(calls_to(&mut e, BASE_BODY_006B3EB0), vec![vec![this, 5, 6]]);
        assert_eq!(e.mem.u32(this + 8), VTABLE_00869F00);
    }
}
