//! `fallout/interface/fopipboymanager.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007fa200 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `007f9840` (Ghidra name
/// `FOPipboyManager::InitLight`), which creates the menu light `Omni01` and sets its
/// colours. The engine map names this address `FOPipboyManager::ReInitialize` from the call
/// graph only (row `cg`), which this translation does not confirm, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `007f9840`'s disassembly ends with `CALL 007f99b0` (which stores a
/// float and ends `RET 4`) and no result is computed (Ghidra types it void).
pub fn fn_007fa200(e: &mut Engine, this: Ptr) {
    e.call(0x007f_9840, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007f_a200, fn_007fa200(Ptr))]
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
    fn fwd_a03_007fa200_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x007f_9840, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x007f_a200, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x007f_9840), vec![vec![0x5000]]);
    }
}
