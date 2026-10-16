//! `fallout shared/run_0058b3f0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0058b450 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00464f40` of unit tesdatahandler, and returns
/// nothing. Parameters: one word, `this`; the disassembly of the function ends in a plain
/// `RET` and passes only `ECX` on. `00464f40`'s translated signature `fn_00464f40(Ptr)`
/// returns nothing.
pub fn fn_0058b450(e: &mut Engine, this: Ptr) {
    e.call(0x0046_4f40, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0058_b450, fn_0058b450(Ptr))]
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
    fn fwd_a03_0058b450_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0046_4f40, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0058_b450, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0046_4f40), vec![vec![0x5000]]);
    }
}
