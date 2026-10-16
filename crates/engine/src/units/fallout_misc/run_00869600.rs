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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0086_9a20, fn_00869a20(Ptr)),
        entry!(0x0086_9b70, fn_00869b70(Ptr)),
        entry!(0x0086_9cc0, fn_00869cc0(Ptr)),
        entry!(0x0086_9e10, fn_00869e10(Ptr)),
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
