//! `fallout/ai/lipsyncbackgroundmanager.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00905c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00c3cea0`, which stores the vtable `010c1554` at
/// `+0`, calls `00aa4060` on the word at `+0x28` and ends with `00c3c620`, and returns
/// nothing. Parameters: one word, `this`; the disassembly of the function ends in a plain
/// `RET` and passes only `ECX` on. `00c3cea0` ends with `CALL 00c3c620` and no result of its
/// own (Ghidra types both void).
pub fn fn_00905c30(e: &mut Engine, this: Ptr) {
    e.call(0x00c3_cea0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0090_5c30, fn_00905c30(Ptr))]
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
    fn fwd_a03_00905c30_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00c3_cea0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0090_5c30, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x00c3_cea0), vec![vec![0x5000]]);
    }
}
