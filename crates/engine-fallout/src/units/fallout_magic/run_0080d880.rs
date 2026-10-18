//! `fallout/magic/run_0080d880` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0080d930 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00803ed0`, a destructor with an exception frame
/// that stores the vtable `0107935c` and tears down the sound handle and the list at
/// `+0x40`, and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `00803ed0`'s disassembly ends
/// with its exception-frame unlink and returns no value (Ghidra types it void).
pub fn fn_0080d930(e: &mut Engine, this: Ptr) {
    e.call(0x0080_3ed0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0080_d930, fn_0080d930(Ptr))]
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
    fn fwd_a03_0080d930_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0080_3ed0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0080_d930, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0080_3ed0), vec![vec![0x5000]]);
    }
}
