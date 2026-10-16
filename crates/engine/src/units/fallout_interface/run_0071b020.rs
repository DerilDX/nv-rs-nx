//! `fallout/interface/run_0071b020` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0071b020 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0071dec0` of unit run_0071dd50 (the string
/// list's destructor), and returns nothing. Parameters: one word, `this`; the disassembly of
/// the function ends in a plain `RET` and passes only `ECX` on. `0071dec0`'s translated
/// signature `fn_0071dec0(Ptr<NiTPointerList>)` returns nothing.
pub fn fn_0071b020(e: &mut Engine, this: Ptr) {
    e.call(0x0071_dec0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0071_b020, fn_0071b020(Ptr))]
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
    fn fwd_a03_0071b020_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0071_dec0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0071_b020, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0071_dec0), vec![vec![0x5000]]);
    }
}
