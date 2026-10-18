//! `(unplaced)/run_0061f220` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0061f440 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0061cad0` of unit testopicinfo (clears a
/// response list), and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `0061cad0`'s translated
/// signature `fn_0061cad0(Ptr)` returns nothing.
pub fn fn_0061f440(e: &mut Engine, this: Ptr) {
    e.call(0x0061_cad0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0061_f440, fn_0061f440(Ptr))]
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
    fn fwd_a03_0061f440_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0061_cad0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0061_f440, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0061_cad0), vec![vec![0x5000]]);
    }
}
