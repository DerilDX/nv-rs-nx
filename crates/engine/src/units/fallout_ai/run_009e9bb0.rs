//! `fallout/ai/run_009e9bb0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009e9c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `009e9c30`, which stores the vtable `01092930` at
/// `+0` and frees the block held at `+4` through the cdecl `004ede70`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `009e9c30`'s disassembly ends with the stack cleanup of that call
/// (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_009e9c10(e: &mut Engine, this: Ptr) {
    e.call(0x009e_9c30, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009e_9c10, fn_009e9c10(Ptr))]
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
    fn fwd_a03_009e9c10_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x009e_9c30, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009e_9c10, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x009e_9c30), vec![vec![0x5000]]);
    }
}
