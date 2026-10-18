//! `fallout shared/pathfinding/run_006a68b0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006a68b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `006a68d0`, which calls the smart-pointer
/// destructor `0045cec0` on the member at `this + 8`, and returns nothing. Parameters: one
/// word, `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX`
/// on. `006a68d0` ends right after that call, and `0045cec0`'s translated signature
/// `fn_0045cec0(Ptr)` returns nothing.
pub fn fn_006a68b0(e: &mut Engine, this: Ptr) {
    e.call(0x006a_68d0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006a_68b0, fn_006a68b0(Ptr))]
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
    fn fwd_a03_006a68b0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x006a_68d0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006a_68b0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x006a_68d0), vec![vec![0x5000]]);
    }
}
