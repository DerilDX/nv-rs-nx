//! `fallout/ai/run_0096a850` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0096a930 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0096ad00`, which stores the vtable `0108c0c8` at
/// `+0` and frees the block held at `+4` through the cdecl `004ede70`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `0096ad00`'s disassembly ends with the stack cleanup of that call
/// (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_0096a930(e: &mut Engine, this: Ptr) {
    e.call(0x0096_ad00, &args![this]);
}

// Translated from 0096a950 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0096a930` of this unit, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `0096a930` is a function of this unit and returns nothing (see its
/// own doc).
pub fn fn_0096a950(e: &mut Engine, this: Ptr) {
    fn_0096a930(e, this);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0096_a930, fn_0096a930(Ptr)),
        entry!(0x0096_a950, fn_0096a950(Ptr)),
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
    fn fwd_a03_0096a930_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0096_ad00, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0096_a930, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0096_ad00), vec![vec![0x5000]]);
    }

    #[test]
    fn fwd_a03_0096a950_runs_the_unit_function_and_through_it_the_destructor() {
        let mut e = Engine::new();
        e.register(0x0096_ad00, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0096_a950, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0096_ad00), vec![vec![0x5000]]);
        // The call to the unit's own function is direct, not through the table.
        assert!(calls_to(&e, 0x0096_a930).is_empty());
    }
}
