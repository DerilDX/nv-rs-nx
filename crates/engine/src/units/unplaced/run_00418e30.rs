//! `(unplaced)/run_00418e30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00418f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0045cec0`, the smart-pointer destructor of unit
/// run_0045cb90, and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `0045cec0`'s translated
/// signature `fn_0045cec0(Ptr)` returns nothing.
pub fn fn_00418f40(e: &mut Engine, this: Ptr) {
    e.call(0x0045_cec0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0041_8f40, fn_00418f40(Ptr))]
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
    fn fwd_a03_00418f40_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0045_cec0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0041_8f40, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0045_cec0), vec![vec![0x5000]]);
    }
}
