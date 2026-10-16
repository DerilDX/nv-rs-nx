//! `fallout shared/run_005f0040` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005f0380 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `005f0820`, which calls `005f0860` for each index
/// 0 to 0x15, and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `005f0820`'s disassembly is a
/// counting loop that ends on its exit test without loading a result (Ghidra types it void).
pub fn fn_005f0380(e: &mut Engine, this: Ptr) {
    e.call(0x005f_0820, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005f_0380, fn_005f0380(Ptr))]
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
    fn fwd_a03_005f0380_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x005f_0820, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005f_0380, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x005f_0820), vec![vec![0x5000]]);
    }
}
