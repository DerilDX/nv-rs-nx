//! `fallout/audio/run_008373f0` (Xbox PDB source unit), subsystem `fallout/audio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008373f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00837410`, which frees the chain of nodes
/// starting at `+4` (through `00837480`) and zeroes the float at `+0`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00837410`'s disassembly ends on `FLDZ; FSTP float ptr [EDX]` (a
/// store) with no result (Ghidra types it void).
pub fn fn_008373f0(e: &mut Engine, this: Ptr) {
    e.call(0x0083_7410, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0083_73f0, fn_008373f0(Ptr))]
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
    fn fwd_a03_008373f0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0083_7410, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0083_73f0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0083_7410), vec![vec![0x5000]]);
    }
}
