//! `(unplaced)/run_0065cfb0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0065cfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0065d6e0`, a destructor with an exception frame
/// that calls `0064df10` on the word at `+0` with 1 and then `00483710`, and returns
/// nothing. Parameters: one word, `this`; the disassembly of the function ends in a plain
/// `RET` and passes only `ECX` on. `0065d6e0`'s disassembly ends with its exception-frame
/// unlink after the empty `00483710`; no result is computed (Ghidra types it void).
pub fn fn_0065cfb0(e: &mut Engine, this: Ptr) {
    e.call(0x0065_d6e0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0065_cfb0, fn_0065cfb0(Ptr))]
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
    fn fwd_a03_0065cfb0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0065_d6e0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0065_cfb0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0065_d6e0), vec![vec![0x5000]]);
    }
}
