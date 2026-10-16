//! `fallout/interface/run_007f7770` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007f7770 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `007004e0` of unit run_00700480, which is itself
/// a forwarder to `00700500`, and returns nothing. Parameters: one word, `this`; the
/// disassembly of the function ends in a plain `RET` and passes only `ECX` on. `007004e0` is
/// translated in this family as `fn_007004e0(Ptr)` and returns nothing.
pub fn fn_007f7770(e: &mut Engine, this: Ptr) {
    e.call(0x0070_04e0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007f_7770, fn_007f7770(Ptr))]
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
    fn fwd_a03_007f7770_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0070_04e0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x007f_7770, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0070_04e0), vec![vec![0x5000]]);
    }
}
