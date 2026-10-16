//! `fallout/ai/run_008caa70` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008cab00 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0044fb00`, which calls the import slot
/// `00fdf058` (Ghidra: `DeleteCriticalSection`) on `this`, and returns nothing. Parameters:
/// one word, `this`; the disassembly of the function ends in a plain `RET` and passes only
/// `ECX` on. `0044fb00` ends right after that call; `DeleteCriticalSection` returns nothing
/// and Ghidra types `0044fb00` void.
pub fn fn_008cab00(e: &mut Engine, this: Ptr) {
    e.call(0x0044_fb00, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008c_ab00, fn_008cab00(Ptr))]
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
    fn fwd_a03_008cab00_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0044_fb00, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008c_ab00, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0044_fb00), vec![vec![0x5000]]);
    }
}
