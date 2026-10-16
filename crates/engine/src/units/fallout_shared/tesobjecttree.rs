//! `fallout shared/tesobjecttree.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0051cfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00a5e930`, a destructor that stores the vtable
/// `0109b5ac`, releases the references that vtable slot +0xec returns for each of the
/// entries counted by the half-word at `+0xa6`, and tail-jumps into the base destructor
/// `00a5a390`, and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `00a5e930` ends in `JMP
/// 00a5a390` (which itself ends in `JMP 00a5c240` and then `JMP 00a5d3d0`); no value is
/// computed for the caller and Ghidra types it void.
pub fn fn_0051cfe0(e: &mut Engine, this: Ptr) {
    e.call(0x00a5_e930, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051_cfe0, fn_0051cfe0(Ptr))]
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
    fn fwd_a03_0051cfe0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00a5_e930, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0051_cfe0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x00a5_e930), vec![vec![0x5000]]);
    }
}
