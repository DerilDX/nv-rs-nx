//! `fallout shared/speedtree/bstreenode.cpp` (Xbox PDB source unit), subsystem `fallout shared/speedtree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0066d470 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0066d410`, which forwards `this` to `00c6c290`
/// (a destructor with an exception frame), and returns nothing. Parameters: one word,
/// `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX` on.
/// `0066d410` ends right after that call and `00c6c290` ends with its frame unlink and
/// register restores, loading no result; Ghidra types both void.
pub fn fn_0066d470(e: &mut Engine, this: Ptr) {
    e.call(0x0066_d410, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0066_d470, fn_0066d470(Ptr))]
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
    fn fwd_a03_0066d470_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0066_d410, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0066_d470, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0066_d410), vec![vec![0x5000]]);
    }
}
